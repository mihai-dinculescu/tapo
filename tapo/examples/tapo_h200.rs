//! H200 Example
use log::info;
use tapo::ApiClient;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h200(ip_address)
        .await?;

    let device_info_json = hub.get_device_info_json().await?;
    info!("Device info (JSON): {device_info_json}");

    let device_info = hub.get_device_info().await?;
    info!("Device info: {device_info:?}");

    let child_device_list_json = hub.get_child_device_list_json(0).await?;
    info!("Child device list (JSON): {child_device_list_json}");

    Ok(())
}
