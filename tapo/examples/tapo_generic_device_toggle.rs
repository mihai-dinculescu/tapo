/// Toggle Generic Device Example
use std::env;

use log::{info, warn, LevelFilter};
use tapo::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let device = ApiClient::new(tapo_username, tapo_password)
        .generic_device(ip_address)
        .await?;

    let device_info = device.get_device_info().await?;

    match device_info.device_on {
        Some(true) => {
            info!("Device is on. Turning it off...");
            device.off().await?;
        }
        Some(false) => {
            info!("Device is off. Turning it on...");
            device.on().await?;
        }
        None => {
            warn!("This device does not support on/off functionality.");
        }
    }

    Ok(())
}
