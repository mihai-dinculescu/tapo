use tapo::{ApiClient, DiscoveryResult, StreamExt as _};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{
    ChildDevice, Device, DevicesList, DiscoveryError, GetCapability, SetCapability,
    UnsupportedDevice,
};

pub async fn get_devices(config: &AppConfig) -> Result<DevicesList, TapoMcpError> {
    let mut devices = Vec::new();
    let mut unsupported = Vec::new();
    let mut errors = Vec::new();

    tracing::info!(
        discovery_target = config.discovery_target.as_str(),
        discovery_timeout = config.discovery_timeout,
        "Discovering devices",
    );

    let api_client = ApiClient::new(config.username.clone(), config.password.clone());
    let mut discovery = api_client
        .discover_devices(config.discovery_target.clone(), config.discovery_timeout)
        .await?;

    while let Some(discovery_result) = discovery.next().await {
        match discovery_result {
            Ok(device) => {
                let id = device.device_id().to_string();
                let name = device.nickname().to_string();
                let model = device.model().to_string();
                let ip = device.ip().to_string();

                let child_result: Option<Result<Vec<ChildDevice>, _>> = match &device {
                    DiscoveryResult::PowerStrip { handler, .. } => {
                        Some(handler.get_child_device_list().await.map(|list| {
                            list.into_iter()
                                .map(|c| ChildDevice {
                                    id: c.device_id,
                                    name: c.nickname,
                                    model: c.model,
                                    set_capabilities: vec![SetCapability::OnOff],
                                    get_capabilities: vec![GetCapability::DeviceInfo],
                                })
                                .collect()
                        }))
                    }
                    DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => {
                        Some(handler.get_child_device_list().await.map(|list| {
                            list.into_iter()
                                .map(|c| ChildDevice {
                                    id: c.device_id,
                                    name: c.nickname,
                                    model: c.model,
                                    set_capabilities: vec![SetCapability::OnOff],
                                    get_capabilities: vec![GetCapability::DeviceInfo],
                                })
                                .collect()
                        }))
                    }
                    _ => None,
                };

                let children = match child_result {
                    Some(Ok(list)) => list,
                    Some(Err(err)) => {
                        tracing::warn!(%err, ip = %ip, "Failed to get child device list");
                        errors.push(DiscoveryError {
                            ip: ip.clone(),
                            message: format!("child device discovery failed: {err}"),
                        });
                        vec![]
                    }
                    None => vec![],
                };

                if matches!(
                    device,
                    DiscoveryResult::GenericDevice { .. } | DiscoveryResult::Hub { .. }
                ) {
                    unsupported.push(UnsupportedDevice { ip, model });
                    continue;
                }

                let set_capabilities = match &device {
                    DiscoveryResult::ColorLight { .. }
                    | DiscoveryResult::RgbLightStrip { .. }
                    | DiscoveryResult::RgbicLightStrip { .. } => {
                        vec![
                            SetCapability::Brightness,
                            SetCapability::Color,
                            SetCapability::OnOff,
                        ]
                    }
                    DiscoveryResult::Light { .. } => {
                        vec![SetCapability::Brightness, SetCapability::OnOff]
                    }
                    DiscoveryResult::Plug { .. } | DiscoveryResult::PlugEnergyMonitoring { .. } => {
                        vec![SetCapability::OnOff]
                    }
                    // Power strip parents have no set capabilities;
                    // only their children do.
                    _ => vec![],
                };

                tracing::debug!(name, model, ip, "Found device");
                devices.push(Device {
                    id,
                    name,
                    model,
                    ip,
                    set_capabilities,
                    get_capabilities: vec![GetCapability::DeviceInfo],
                    children,
                });
            }
            Err(err) => {
                tracing::warn!(%err, "Error discovering device");
                errors.push(DiscoveryError {
                    ip: err.ip.clone(),
                    message: err.source.to_string(),
                });
            }
        }
    }

    tracing::info!("Discovery complete");

    Ok(DevicesList {
        devices,
        unsupported,
        errors,
    })
}
