//! H200 Example

use log::info;
use tapo::ApiClient;
use tapo::responses::ChildDeviceHubResult;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h200(ip_address)
        .await?;

    let device_info = hub.get_device_info().await?;
    info!("Device info: {device_info:?}");

    info!("Getting child devices...");
    let child_device_list = hub.get_child_device_list().await?;

    for child in child_device_list {
        match child {
            ChildDeviceHubResult::KE100(device) => {
                info!(
                    "Found KE100 child device with nickname: {}, id: {}, current temperature: {} {:?} and target temperature: {} {:?}.",
                    device.nickname,
                    device.device_id,
                    device.current_temperature,
                    device.temperature_unit,
                    device.target_temperature,
                    device.temperature_unit,
                );
            }
            ChildDeviceHubResult::S200(device) => {
                info!(
                    "Found S200B/S200D child device with nickname: {}, id: {}.",
                    device.nickname, device.device_id
                );
            }
            ChildDeviceHubResult::S210(device) => {
                info!(
                    "Found S210 child device with nickname: {}, id: {}, device_on: {}.",
                    device.nickname, device.device_id, device.device_on
                );
            }
            ChildDeviceHubResult::T100(device) => {
                info!(
                    "Found T100 child device with nickname: {}, id: {}, detected: {}.",
                    device.nickname, device.device_id, device.detected
                );
            }
            ChildDeviceHubResult::T110(device) => {
                info!(
                    "Found T110 child device with nickname: {}, id: {}, open: {}.",
                    device.nickname, device.device_id, device.open
                );
            }
            ChildDeviceHubResult::T300(device) => {
                info!(
                    "Found T300 child device with nickname: {}, id: {}, in_alarm: {}, water_leak_status: {:?}.",
                    device.nickname, device.device_id, device.in_alarm, device.water_leak_status
                );
            }
            ChildDeviceHubResult::T31X(device) => {
                info!(
                    "Found T310/T315 child device with nickname: {}, id: {}, temperature: {} {:?}, humidity: {}%.",
                    device.nickname,
                    device.device_id,
                    device.current_temperature,
                    device.temperature_unit,
                    device.current_humidity,
                );
            }
            ChildDeviceHubResult::Other(device) => {
                info!(
                    "Found unsupported child device with nickname: {}, id: {}, model: {}.",
                    device.nickname, device.device_id, device.model
                );
            }
        }
    }

    Ok(())
}
