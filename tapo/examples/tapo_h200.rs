//! H200 Example

use log::info;
use tapo::responses::ChildDeviceHubResult;
use tapo::{ApiClient, HubDevice};

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

    let child_device_component_list = hub.get_child_device_component_list().await?;
    info!("Child device component list: {child_device_component_list:?}");

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
                let s200 = hub
                    .s200(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let trigger_logs = s200.get_trigger_logs(5, 0).await?;

                info!(
                    "Found S200B/S200D child device with nickname: {}, id: {}, last 5 trigger logs: {:?}.",
                    device.nickname, device.device_id, trigger_logs
                );
            }
            ChildDeviceHubResult::S210(device) => {
                let s210 = hub
                    .s210(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let device_usage = s210.get_device_usage().await?;

                info!(
                    "Found S210 child device with nickname: {}, id: {}, device_on: {}, device usage: {:?}.",
                    device.nickname, device.device_id, device.device_on, device_usage
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
            ChildDeviceHubResult::T31X(device) => {
                let t31x = hub
                    .t31x(HubDevice::ByDeviceId(device.device_id.clone()))
                    .await?;
                let temperature_humidity_records = t31x.get_temperature_humidity_records().await?;

                info!(
                    "Found T310/T315 child device with nickname: {}, id: {}, temperature: {} {:?}, humidity: {}%, earliest temperature and humidity record available: {:?}.",
                    device.nickname,
                    device.device_id,
                    device.current_temperature,
                    device.temperature_unit,
                    device.current_humidity,
                    temperature_humidity_records.records.first()
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

    let general_device_list = hub.get_general_device_list().await?;

    for general_device in general_device_list {
        info!(
            "Found general device with alias: {}, id: {}, model: {}, hub storage enabled: {}.",
            general_device.alias,
            general_device.device_id,
            general_device.device_model,
            general_device.hub_storage_enabled
        );

        if !general_device.hub_storage_enabled {
            continue;
        }

        let end_date = chrono::Utc::now().date_naive();
        let start_date = end_date - chrono::Duration::days(7);

        let recording_dates = hub
            .search_date_with_video(
                start_date,
                end_date,
                general_device.device_id.clone(),
                general_device.mac.clone(),
            )
            .await?;
        info!(
            "{} has recordings stored on the hub on the following dates: {recording_dates:?}.",
            general_device.alias
        );

        if let Some(date) = recording_dates.last() {
            let day_start = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as u64;
            let day_end = day_start + 24 * 60 * 60 - 1;

            let recordings = hub
                .search_video_with_utc(
                    day_start,
                    day_end,
                    general_device.device_id.clone(),
                    general_device.mac.clone(),
                )
                .await?;
            info!(
                "{} has {} recordings on {date}. First: {:?}.",
                general_device.alias,
                recordings.len(),
                recordings.first()
            );
        }
    }

    Ok(())
}
