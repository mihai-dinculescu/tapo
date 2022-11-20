/// L510 Example
use std::{env, thread, time::Duration};

use log::{info, LevelFilter};
use tapo::{ApiClient, L510};

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

    let device = ApiClient::<L510>::new(ip_address, tapo_username, tapo_password, true).await?;

    info!("Turning device on...");
    device.on().await?;

    info!("Setting the brightness to 30%...");
    device.set_brightness(30).await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Using the `set` API to set multiple properties in a single request...");
    device.set().on().brightness(50)?.send().await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Turning device off...");
    device.off().await?;

    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    let device_usage = device.get_device_usage().await?;
    info!("Device usage: {device_usage:?}");

    Ok(())
}
