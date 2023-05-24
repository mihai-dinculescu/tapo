/// H100 Example
use std::env;

use log::{info, LevelFilter};
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

    let ip_address = env::var("IP_ADDRESS")?;
    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;

    let device = ApiClient::new(ip_address, tapo_username, tapo_password)?
        .h100()
        .login()
        .await?;

    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    let child_device_list = device.get_child_device_list().await?;
    info!("Child device list: {child_device_list:?}");

    Ok(())
}
