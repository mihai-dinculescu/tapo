//! H100 Example

use log::{info, LevelFilter};
use std::env;
use std::time::Duration;
use tapo::requests::{AlarmDuration, AlarmRingtone, AlarmVolume};
use tapo::responses::ChildDeviceHubResult;
use tapo::{ApiClient, HubDevice};

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

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h100(ip_address)
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
            ChildDeviceHubResult::S200B(device) => {
                let s200b = hub
                    .s200b(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let trigger_logs = s200b.get_trigger_logs(5, 0).await?;

                info!(
                    "Found S200B child device with nickname: {}, id: {}, last 5 trigger logs: {:?}.",
                    device.nickname, device.device_id, trigger_logs
                );
            }
            ChildDeviceHubResult::T100(device) => {
                let t100 = hub
                    .t100(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let trigger_logs = t100.get_trigger_logs(5, 0).await?;

                info!(
                    "Found T100 child device with nickname: {}, id: {}, detected: {}, last 5 trigger logs: {:?}.",
                    device.nickname, device.device_id, device.detected, trigger_logs
                );
            }
            ChildDeviceHubResult::T110(device) => {
                let t110 = hub
                    .t110(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let trigger_logs = t110.get_trigger_logs(5, 0).await?;

                info!(
                    "Found T110 child device with nickname: {}, id: {}, open: {}, last 5 trigger logs: {:?}.",
                    device.nickname, device.device_id, device.open, trigger_logs
                );
            }
            ChildDeviceHubResult::T300(device) => {
                let t300 = hub
                    .t300(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let trigger_logs = t300.get_trigger_logs(5, 0).await?;

                info!(
                    "Found T300 child device with nickname: {}, id: {}, in_alarm: {}, water_leak_status: {:?}, last 5 trigger logs: {:?}.",
                    device.nickname,
                    device.device_id,
                    device.in_alarm,
                    device.water_leak_status,
                    trigger_logs
                );
            }
            ChildDeviceHubResult::T310(device) | ChildDeviceHubResult::T315(device) => {
                let t31x = hub
                    .t315(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let temperature_humidity_records = t31x.get_temperature_humidity_records().await?;

                info!(
                    "Found T31X child device with nickname: {}, id: {}, temperature: {} {:?}, humidity: {}%, earliest temperature and humidity record available: {:?}.",
                    device.nickname,
                    device.device_id,
                    device.current_temperature,
                    device.temperature_unit,
                    device.current_humidity,
                    temperature_humidity_records.records.first()
                );
            }
            _ => {
                info!("Found unsupported device.")
            }
        }
    }

    info!("Triggering the alarm ringtone 'Alarm 1' at a 'Low' volume for '3 Seconds'...");
    hub.play_alarm(
        AlarmRingtone::Alarm1,
        AlarmVolume::Low,
        AlarmDuration::Seconds(3),
    )
    .await?;

    let device_info = hub.get_device_info().await?;
    info!("Is device ringing?: {:?}", device_info.in_alarm);

    info!("Stopping the alarm after 1 Second...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    hub.stop_alarm().await?;

    let device_info = hub.get_device_info().await?;
    info!("Is device ringing?: {:?}", device_info.in_alarm);

    Ok(())
}
