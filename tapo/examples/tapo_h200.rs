//! H200 Example

use log::{error, info};
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

    #[cfg(feature = "debug")]
    {
        let component_list = hub.get_component_list().await?;
        info!("Component list: {component_list:?}");
    }

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

    // The first recording found for each camera, to download at the end.
    let mut recordings_to_download = Vec::new();

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

        let end_time = chrono::Utc::now();
        let start_time = end_time - chrono::Duration::days(7);

        let recording_dates = hub
            .get_recording_dates(general_device.device_id.clone(), start_time, end_time)
            .await?;
        info!(
            "{} has recordings stored on the hub on the following dates: {recording_dates:?}.",
            general_device.alias
        );

        if let Some(recording_date) = recording_dates.last() {
            let recordings = hub
                .get_recordings(
                    general_device.device_id.clone(),
                    recording_date.start_time,
                    recording_date.end_time,
                )
                .await?;
            info!(
                "{} has {} recordings on {}. First: {:?}.",
                general_device.alias,
                recordings.len(),
                recording_date.date,
                recordings.first()
            );

            if let Some(recording) = recordings.into_iter().next() {
                recordings_to_download.push((general_device.device_id, recording));
            }
        }
    }

    if recordings_to_download.is_empty() {
        info!("No recording found to download.");
    }

    for (device_id, recording) in recordings_to_download {
        let path = format!(
            "recording_{device_id}_{}.ts",
            recording.start_time.timestamp()
        );
        info!(
            "Downloading the recording from {} to {} of {device_id} to {path}...",
            recording.start_time, recording.end_time
        );

        let mut media = Vec::new();
        match hub
            .download_recording(
                device_id.clone(),
                recording.start_time,
                recording.end_time,
                &mut media,
            )
            .await
        {
            Ok(result) => {
                std::fs::write(&path, &media)?;

                let duration = match result.duration_s {
                    Some(duration_s) => format!("{duration_s:.3} s"),
                    None => "an unknown length".to_string(),
                };
                info!(
                    "Wrote {} bytes to {path}: {duration} of video.",
                    media.len()
                );
            }
            Err(err) => {
                error!("Failed to download the recording of {device_id}: {err:?}");
            }
        }
    }

    Ok(())
}
