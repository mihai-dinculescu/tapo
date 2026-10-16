/// Discover devices on the local network Example
use std::env;

use log::{error, info, warn};
use tapo::ApiClient;
use tapo::{DiscoveryResult, StreamExt};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD"])?;
    let target = env::var("TAPO_DISCOVERY_TARGET").unwrap_or_else(|_| "192.168.1.255".to_string());
    let timeout_s = env::var("TAPO_DISCOVERY_TIMEOUT")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<u64>()
        .unwrap_or(10);

    info!("Discovering Tapo devices on target: {target} for {timeout_s} seconds...");

    let api_client = ApiClient::new(tapo_username, tapo_password);
    let mut discovery = api_client.discover_devices(target, timeout_s).await?;

    // NOTE: This example uses explicit pattern matching to demonstrate all
    // DiscoveryResult variants and the way in which they expose `device_info`
    // and `handler`. For a more concise approach, you can use the
    // accessor methods instead for a few common properties:
    //
    //   let device_id = discovery_result.device_id();
    //   let nickname = discovery_result.nickname();
    //   let model = discovery_result.model();
    //   let address = discovery_result.address();
    //   let device_type = discovery_result.device_type();

    while let Some(discovery_result) = discovery.next().await {
        if let Ok(discovery_result) = discovery_result {
            match discovery_result {
                DiscoveryResult::Light {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::ColorLight {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::RgbLightStrip {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::RgbicLightStrip {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::Plug {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::PlugEnergyMonitoring {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::PowerStrip {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found Power Strip of model {:?} at {ip}:{port}.",
                        device_info.model
                    );
                }
                DiscoveryResult::PowerStripEnergyMonitoring {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found Power Strip with Energy Monitoring of model {:?} at {ip}:{port}.",
                        device_info.model
                    );
                }
                DiscoveryResult::Hub {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::HubIr {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::CameraHub {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::Camera {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::CameraPtz {
                    device_info,
                    handler,
                } => {
                    let (ip, port) = handler.address();
                    info!(
                        "Found {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                DiscoveryResult::Other {
                    device_info,
                    address: (ip, port),
                } => {
                    // If you believe that this device is already supported through one of the existing handlers, or would like to explore adding support for a currently
                    // unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
                    // to start the discussion.
                    warn!(
                        "Found device without a dedicated handler {:?} of model {:?} at {ip}:{port}.",
                        device_info.nickname, device_info.model
                    );
                }
                // `DiscoveryResult` is non-exhaustive. Every type it has today is handled above.
                _ => unreachable!(),
            }
        } else if let Err(e) = discovery_result {
            error!("Error discovering device: {e:?}");
            continue;
        }
    }

    Ok(())
}
