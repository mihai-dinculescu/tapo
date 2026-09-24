//! H110 Siren Example
//!
//! Checks whether the H110 siren can be driven the same way as the H100 one.
//! It prints the raw device info, the component list and the supported ringtones,
//! then plays a short, low volume alarm and stops it.
//!
//! Each step reports its own error instead of aborting, so a single run shows
//! the outcome of every check.
//!
//! Requires the `debug` feature:
//!
//! ```bash
//! cargo run --example tapo_h110_siren --features debug
//! ```
use std::time::Duration;

use log::{error, info};
use tapo::ApiClient;
use tapo::requests::{AlarmDuration, AlarmRingtone, AlarmVolume};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h110(ip_address)
        .await?;

    info!("Getting the raw device info...");
    match hub.get_device_info_json().await {
        Ok(device_info) => info!(
            "Device info JSON: {}",
            serde_json::to_string_pretty(&device_info)?
        ),
        Err(err) => error!("get_device_info_json failed: {err:?}"),
    }

    info!("Getting the component list...");
    match hub.get_component_list().await {
        Ok(components) => {
            for component in &components {
                info!(
                    "Component: {} (ver_code {})",
                    component.id, component.ver_code
                );
            }

            let has_alarm = components.iter().any(|c| c.id.contains("alarm"));
            info!("Component list contains an alarm component: {has_alarm}");
        }
        Err(err) => error!("get_component_list failed: {err:?}"),
    }

    info!("Getting the supported ringtone list...");
    match hub.get_supported_ringtone_list().await {
        Ok(ringtones) => info!("Supported ringtones ({}): {ringtones:?}", ringtones.len()),
        Err(err) => error!("get_supported_ringtone_list failed: {err:?}"),
    }

    info!("Triggering the alarm ringtone 'Alarm 1' at a 'Low' volume for '3 Seconds'...");
    match hub
        .play_alarm(
            AlarmRingtone::Alarm1,
            AlarmVolume::Low,
            AlarmDuration::Seconds(3),
        )
        .await
    {
        Ok(()) => info!("play_alarm succeeded. Did the hub make a sound?"),
        Err(err) => error!("play_alarm failed: {err:?}"),
    }

    match hub.get_device_info_json().await {
        Ok(device_info) => info!(
            "While ringing: in_alarm = {:?}, in_alarm_source = {:?}",
            device_info.get("in_alarm"),
            device_info.get("in_alarm_source")
        ),
        Err(err) => error!("get_device_info_json failed: {err:?}"),
    }

    info!("Stopping the alarm after 1 Second...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    match hub.stop_alarm().await {
        Ok(()) => info!("stop_alarm succeeded."),
        Err(err) => error!("stop_alarm failed: {err:?}"),
    }

    match hub.get_device_info_json().await {
        Ok(device_info) => info!(
            "After stopping: in_alarm = {:?}, in_alarm_source = {:?}",
            device_info.get("in_alarm"),
            device_info.get("in_alarm_source")
        ),
        Err(err) => error!("get_device_info_json failed: {err:?}"),
    }

    Ok(())
}
