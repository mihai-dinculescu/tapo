/// P300 and P306 Example
use std::{env, thread, time::Duration};

use log::info;
use tapo::{ApiClient, Plug};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let power_strip = ApiClient::new(tapo_username, tapo_password)
        .p300(ip_address)
        .await?;

    let device_info = power_strip.get_device_info().await?;
    info!("Device info: {device_info:?}");

    info!("Getting child devices...");
    let child_device_list = power_strip.get_child_device_list().await?;
    info!("Found {} plugs", child_device_list.len());

    for (index, child) in child_device_list.into_iter().enumerate() {
        info!("=== ({}) {} ===", index + 1, child.nickname);
        info!("Device ID: {}", child.device_id);
        info!("State: {}", child.device_on);

        let plug = power_strip.plug(Plug::ByDeviceId(child.device_id)).await?;

        info!("Turning device on...");
        plug.on().await?;

        info!("Waiting 2 seconds...");
        thread::sleep(Duration::from_secs(2));

        info!("Turning device off...");
        plug.off().await?;

        info!("Waiting 2 seconds...");
        thread::sleep(Duration::from_secs(2));
    }

    Ok(())
}
