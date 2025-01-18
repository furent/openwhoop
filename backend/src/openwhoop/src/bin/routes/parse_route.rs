use actix_web::{post, web, HttpResponse};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use serde::Serialize;
use std::io::Write;
use tempfile::NamedTempFile;

fn calculate_time_domain_metrics(rr_intervals: &[u16]) -> (Option<f64>, Option<f64>) {
    if rr_intervals.len() < 2 {
        return (None, None); // Not enough intervals
    }
    let rr: Vec<f64> = rr_intervals.iter().map(|&x| x as f64).collect();
    // diffs
    let mut diffs = Vec::new();
    for w in rr.windows(2) {
        diffs.push(w[1] - w[0]);
    }
    let squared: Vec<f64> = diffs.iter().map(|x| x * x).collect();
    let mean_sq = squared.iter().sum::<f64>() / (squared.len() as f64);
    let rmssd = mean_sq.sqrt();

    // SDNN
    let mean_rr = rr.iter().sum::<f64>() / (rr.len() as f64);
    let var = rr.iter().map(|&x| (x - mean_rr).powi(2)).sum::<f64>() / (rr.len() as f64);
    let sdnn = var.sqrt();

    (Some(rmssd), Some(sdnn))
}

#[derive(Debug)]
pub struct HistoricalRecord {
    pub unix: u32,
    pub heart_rate: u8,
    pub rr: Vec<u16>,
    pub hrv_rmssd: Option<f64>,
}

// Final JSON shape
#[derive(Serialize)]
pub struct ParsedRecord {
    timestamp: u32,
    heart_rate: u8,
    rr_intervals: Vec<u16>,
    hrv_rmssd: Option<f64>,
}

use whoop::WhoopPacket;
use whoop::constants::PacketType;

#[post("/api/parse-history")]
pub async fn parse_history(mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    let mut bin_data = Vec::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            bin_data.extend_from_slice(&data);
        }
    }

    let mut records: Vec<HistoricalRecord> = Vec::new();
    let mut dp = 0_usize;
    while dp < bin_data.len() {
        if dp + 3 > bin_data.len() {
            break; 
        }

        let length_le = u16::from_le_bytes([
            bin_data[dp+1],
            bin_data[dp+2],
        ]);
        let length = length_le as usize + 4; // add CRC length

        if dp + length > bin_data.len() {
            // partial / corrupted => break or return error
            break;
        }

        // Extract sub-slice
        let packet_bytes = &bin_data[dp..dp + length];
        dp += length;

        let pkt = match WhoopPacket::from_data(packet_bytes.to_vec()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to parse whoop packet: {:?}", e);
                continue; 
            }
        };
        if pkt.data.len() < 15 {
            // Not enough data => skip
            continue;
        }

        let unix_le = &pkt.data[4..8];  
        let subsec_le = &pkt.data[8..10];
        let unix = u32::from_le_bytes(unix_le.try_into().unwrap());
        let heart = pkt.data[11];

        let rrnum = pkt.data[15];
        let mut rr = Vec::new();
        if pkt.data.len() >= 24 {
            let rr1 = u16::from_le_bytes(pkt.data[16..18].try_into().unwrap());
            let rr2 = u16::from_le_bytes(pkt.data[18..20].try_into().unwrap());
            let rr3 = u16::from_le_bytes(pkt.data[20..22].try_into().unwrap());
            let rr4 = u16::from_le_bytes(pkt.data[22..24].try_into().unwrap());
            match rrnum {
                1 => rr = vec![rr1],
                2 => rr = vec![rr1, rr2],
                3 => rr = vec![rr1, rr2, rr3],
                4 => rr = vec![rr1, rr2, rr3, rr4],
                0 => rr = vec![],
                _ => {
                    eprintln!("Unknown rrnum: {}", rrnum);
                }
            }
        }

        records.push(HistoricalRecord {
            unix,
            heart_rate: heart,
            rr,
            hrv_rmssd: None,
        });
    }

    let chunk_size = 10;
    for chunk_start in (0..records.len()).step_by(chunk_size) {
        let end = std::cmp::min(chunk_start + chunk_size, records.len());
        let chunk = &mut records[chunk_start..end];

        let mut all_rr = Vec::new();
        for rec in chunk.iter() {
            all_rr.extend(&rec.rr);
        }
        if all_rr.len() >= 2 {
            let (rmssd, _sdnn) = calculate_time_domain_metrics(&all_rr);
            for rec in chunk.iter_mut() {
                rec.hrv_rmssd = rmssd;
            }
        }
    }

    // Convert to JSON
    let results: Vec<ParsedRecord> = records
        .into_iter()
        .map(|r| ParsedRecord {
            timestamp: r.unix,
            heart_rate: r.heart_rate,
            rr_intervals: r.rr,
            hrv_rmssd: r.hrv_rmssd,
        })
        .collect();

    Ok(HttpResponse::Ok().json(results))
}
