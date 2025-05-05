/// L510, L520 and L610 Example
use std::{env, thread, time::Duration};

use log::info;
use tapo::ApiClient;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let device = ApiClient::new(tapo_username, tapo_password)
        .l510(ip_address)
        .await?;

    info!("Turning device on...");
    device.on().await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Setting the brightness to 30%...");
    device.set_brightness(30).await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Turning device off...");
    device.off().await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    let device_usage = device.get_device_usage().await?;
    info!("Device usage: {device_usage:?}");

    Ok(())
}
