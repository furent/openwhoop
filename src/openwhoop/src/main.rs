#[macro_use]
extern crate log;

use std::collections::HashSet;
use std::env;
use std::time::Duration;

use anyhow::anyhow;
use btleplug::{
    api::{BDAddr, Central, Peripheral as _, ScanFilter, Manager as _}, 
    platform::{Adapter, Manager, Peripheral},
};

use clap::{Parser, Subcommand};
use dialoguer::Select;
use dotenv::dotenv;
use openwhoop::{DatabaseHandler, OpenWhoop, WhoopDevice};
use tokio::time::{sleep, Instant};
use whoop::{constants::WHOOP_SERVICE, WhoopPacket};

#[derive(Parser)]
pub struct OpenWhoopCli {
    #[arg(env, long)]
    pub database_url: String,
    #[arg(env, long)]
    pub ble_interface: Option<String>,
    #[clap(subcommand)]
    pub subcommand: OpenWhoopCommand,
}

#[derive(Subcommand)]
pub enum OpenWhoopCommand {
    Scan,
    DownloadHistory {
        #[arg(long, env)]
        whoop_addr: Option<BDAddr>,
    },
    ReRun,
    DetectEvents,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env.
    if let Err(error) = dotenv() {
        println!("{}", error);
    }
    
    // If running on macOS, remove the ble_interface variable so that Linux‑only configuration isn’t used.
    // On macos shows all BLE addresses as 00:00:00:00:00:00
    // We remove the BLE interface and BLE_INTEFACE from the .env, so that we can use 
    // the interactive shell in the cli on the macos platform
    if cfg!(target_os = "macos") {
        // Every BLE address is 00:00:00:00:00, so we cant filter for it.
        env::remove_var("BLE_INTERFACE"); 
        env::remove_var("WHOOP_ADDR");
    }


    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_module("sqlx::query", log::LevelFilter::Off)
        .init();

    let cli = OpenWhoopCli::parse();
    let db_handler = DatabaseHandler::new(cli.database_url).await;

    let manager = Manager::new().await?;
    let adapter = match cli.ble_interface {
        Some(interface) => {
            let adapters = manager.adapters().await?;
            let mut c_adapter = Err(anyhow!("Adapter: `{}` not found", interface));
            for adapter in adapters {
                let name = adapter.adapter_info().await?;
                if name.starts_with(&interface) {
                    c_adapter = Ok(adapter);
                    break;
                }
            }
            c_adapter?
        }
        None => {
            let adapters = manager.adapters().await?;
            adapters
                .into_iter()
                .next()
                .ok_or(anyhow!("No BLE adapters found"))?
        }
    };

    match cli.subcommand {
        OpenWhoopCommand::Scan => {
            let _ = scan_command(adapter, None).await?;
            Ok(())
        }
        OpenWhoopCommand::DownloadHistory { whoop_addr } => {
            // On macOS (or when no address is provided), 
            // the scan_command will prompt for a selection.
            // Maybe we want to write the name after first selection into .env?
            let peripheral = scan_command(adapter, whoop_addr).await?;
            let mut whoop = WhoopDevice::new(peripheral, db_handler);

            whoop.connect().await?;
            whoop.initialize().await?;

            if let Err(e) = whoop.sync_history().await {
                error!("{}", e);
            }

            loop {
                if let Ok(true) = whoop.is_connected().await {
                    whoop
                        .send_command(WhoopPacket::exit_high_freq_sync())
                        .await?;
                    break;
                } else {
                    whoop.connect().await?;
                    sleep(Duration::from_secs(1)).await;
                }
            }

            Ok(())
        }
        OpenWhoopCommand::ReRun => {
            let whoop = OpenWhoop::new(db_handler.clone());
            let mut id = 0;
            loop {
                let packets = db_handler.get_packets(id).await?;
                if packets.is_empty() {
                    break;
                }

                for packet in packets {
                    id = packet.id;
                    whoop.handle_packet(packet).await?;
                }

                println!("{}", id);
            }

            Ok(())
        }
        OpenWhoopCommand::DetectEvents => {
            let whoop = OpenWhoop::new(db_handler);
            whoop.detect_sleeps().await?;
            Ok(())
        }
    }
}

/// Scans for devices advertising the WHOOP_SERVICE.
///
/// If a peripheral address is provided, this function waits until that specific device is found.
/// Otherwise, it scans for a fixed duration and then presents the user with an interactive
/// selection prompt
///
/// **New functionality on macOS:**
///
/// Because macOS reports the same dummy BLE address for all devices (00:00:00:00:00:00),
/// we build a composte key using both the device's local name and its unique ID: peripheral.id
/// Thi sunique identifier differentiates devices even when their advertised adress is the same.

async fn scan_command(
    adapter: Adapter,
    peripheral_addr: Option<BDAddr>,
) -> anyhow::Result<Peripheral> {
    adapter
        .start_scan(ScanFilter {
            services: vec![WHOOP_SERVICE],
        })
        .await?;

    // Just as before: if an address is provided, wait for that device.
    if let Some(addr) = peripheral_addr {
        loop {
            let peripherals = adapter.peripherals().await?;
            for peripheral in peripherals {
                if let Some(props) = peripheral.properties().await? {
                    if props.address == addr {
                        return Ok(peripheral);
                    }
                }
            }
            sleep(Duration::from_secs(1)).await;
        }

    // Now new for macos and selection dialogue:
    } else {
        let mut devices = Vec::new();
        let mut seen = HashSet::new();
        let scan_duration = Duration::from_secs(10);
        let start = Instant::now();

        println!("Scanning for devices for {} seconds...", scan_duration.as_secs());
        while Instant::now().duration_since(start) < scan_duration {
            let peripherals = adapter.peripherals().await?;
            for peripheral in peripherals {
                if let Some(props) = peripheral.properties().await? {
                    if !props.services.contains(&WHOOP_SERVICE) {
                        continue;
                    }
                    // Use the peripheral's local name and its unique id, or Unkown
                    let local_name = props.local_name.unwrap_or_else(|| "Unknown".to_string());
                    let unique_id = peripheral.id(); // This is unique even on macOS
                    let key = (local_name.clone(), unique_id);
                    if seen.insert(key) {
                        devices.push(peripheral);
                    }
                }
            }
            sleep(Duration::from_secs(1)).await;
        }

        if devices.is_empty() {
            return Err(anyhow!("No devices found"));
        }

        // Now we make a list of labels for user selection.
        // Label is: <disvocered_name> : <Address>
        // Because we only have 00:00:00:00:00 on macos
        // we take a uniqde id and push it in the list
        let items = {
            let mut list = Vec::new();
            for device in &devices {
                if let Some(props) = device.properties().await? {
                    let local_name = props.local_name.unwrap_or_else(|| "Unknown".to_string());
                    list.push(format!("{} ({})", local_name, props.address));
                }
            }
            list
        };

        let selection = tokio::task::spawn_blocking(move || {
            Select::new()
                .with_prompt("Select a device")
                .items(&items)
                .default(0)
                .interact()
        })
        .await??;

        devices
            .into_iter()
            .nth(selection)
            .ok_or(anyhow!("Invalid selection"))
    }
}

