use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use env_logger;
use openwhoop::{DatabaseHandler, Whoop};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load env vars
    if let Err(error) = dotenv() {
        println!("dotenv error: {}", error);
    }

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_module("sqlx::query", log::LevelFilter::Off)
        .init();

    // 2. Parse CLI arguments
    let cli = OpenWhoop::parse(); // assumes you have an OpenWhoop struct from clap

    // 3. Setup DB
    let db_handler = DatabaseHandler::new(cli.database_url).await?;

    // 4. Setup BLE manager/adapter
    let manager = btleplug::platform::Manager::new().await?;
    // ... the rest of your scanning/downloading logic ...

    match cli.subcommand {
        OpenWhoopCommand::Scan => {
            // run scan
        }
        OpenWhoopCommand::DownloadHistory { whoop_addr } => {
            // connect + download
        }
    }

    Ok(())
}
