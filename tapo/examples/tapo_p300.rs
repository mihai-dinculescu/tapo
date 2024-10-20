/// P300 and P304 Example
use std::{env, thread, time::Duration};

use log::{info, LevelFilter};
use tapo::{ApiClient, Plug};

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

    let power_strip = ApiClient::new(tapo_username, tapo_password)
        .p300(ip_address)
        .await?;

    let device_info = power_strip.get_device_info().await?;
    info!("Device info: {device_info:?}");

    info!("Getting child devices...");
    let child_device_list = power_strip.get_child_device_list().await?;

    for child in child_device_list {
        info!(
            "Found plug with nickname: {}, id: {}, state: {}.",
            child.nickname, child.device_id, child.device_on,
        );

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
