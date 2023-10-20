/// KE100 TRV Example
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

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;
    // ID of the KE100 device. Can be obtained from executing `get_child_device_component_list_json()`` on the hub device.
    let device_id = env::var("DEVICE_ID")?;
    let target_temp: u8 = env::var("TEMPERATURE")?.parse().unwrap();

    let hub = ApiClient::new(tapo_username, tapo_password)?
        .h100(ip_address)
        .await?;

    // Get a handler for the child device
    let device = hub.ke100(device_id);

    // Get the device info of the child device
    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    // Set temperature on target device
    device.set_temperature(target_temp).await?;

    // Get the device info of the child device
    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    Ok(())
}
