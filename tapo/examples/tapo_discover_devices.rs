/// Discover devices on the local network Example
use std::env;

use log::{error, info, warn};
use tapo::ApiClient;
use tapo::{DiscoveryResult, StreamExt};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let target = env::var("TAPO_DISCOVERY_TARGET").unwrap_or_else(|_| "192.168.1.255".to_string());
    let timeout_s = env::var("TAPO_DISCOVERY_TIMEOUT")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<u64>()
        .unwrap_or(10);

    info!("Discovering Tapo devices on target: {target} for {timeout_s} seconds...");

    let api_client = ApiClient::new(tapo_username, tapo_password);
    let mut discovery = api_client.discover_devices(target, timeout_s).await?;

    while let Some(discovery_result) = discovery.next().await {
        if let Ok(device) = discovery_result {
            match device {
                DiscoveryResult::GenericDevice {
                    device_info,
                    handler: _,
                } => {
                    // If you believe this device is already supported, or would like to explore adding support for a currently
                    // unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
                    // to start the discussion.
                    warn!(
                        "Found Unsupported Device {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::Light {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::ColorLight {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::RgbLightStrip {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::RgbicLightStrip {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::Plug {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::PlugEnergyMonitoring {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::PowerStrip {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found Power Strip of model {:?} at IP address {:?}.",
                        device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::PowerStripEnergyMonitoring {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found Power Strip with Energy Monitoring of model {:?} at IP address {:?}.",
                        device_info.model, device_info.ip
                    );
                }
                DiscoveryResult::Hub {
                    device_info,
                    handler: _,
                } => {
                    info!(
                        "Found {:?} of model {:?} at IP address {:?}.",
                        device_info.nickname, device_info.model, device_info.ip
                    );
                }
            }
        } else if let Err(e) = discovery_result {
            error!("Error discovering device: {e:?}");
            continue;
        }
    }

    Ok(())
}
