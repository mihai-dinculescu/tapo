use tapo::responses::ChildDeviceHubResult;
use tapo::{ApiClient, DiscoveryResult, StreamExt as _};
use tokio::task::JoinSet;

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{
    ChildDevice, Device, DevicesList, DiscoveryError, GetCapability, SetCapability,
    UnsupportedDevice,
};

pub async fn get_devices(config: &AppConfig) -> Result<DevicesList, TapoMcpError> {
    tracing::info!(
        discovery_target = config.discovery_target.as_str(),
        discovery_timeout = config.discovery_timeout,
        "Discovering devices",
    );

    let api_client = ApiClient::new(config.username.clone(), config.password.clone());
    let mut discovery = api_client
        .discover_devices(config.discovery_target.clone(), config.discovery_timeout)
        .await?;

    let mut errors = Vec::new();
    let mut joinset: JoinSet<DeviceOutcome> = JoinSet::new();

    while let Some(discovery_result) = discovery.next().await {
        match discovery_result {
            Ok(device) => {
                joinset.spawn(process_device(device));
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

    let mut devices = Vec::new();
    let mut unsupported = Vec::new();
    while let Some(joined) = joinset.join_next().await {
        match joined.expect("device-processing task panicked") {
            DeviceOutcome::Device {
                device,
                child_error,
            } => {
                if let Some(err) = child_error {
                    errors.push(err);
                }
                devices.push(device);
            }
            DeviceOutcome::Unsupported(unsupported_device) => {
                unsupported.push(unsupported_device);
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

enum DeviceOutcome {
    Device {
        device: Device,
        child_error: Option<DiscoveryError>,
    },
    Unsupported(UnsupportedDevice),
}

async fn process_device(device: DiscoveryResult) -> DeviceOutcome {
    let id = device.device_id().to_string();
    let name = device.nickname().to_string();
    let model = device.model().to_string();
    let ip = device.ip().to_string();

    if matches!(device, DiscoveryResult::Other { .. }) {
        return DeviceOutcome::Unsupported(UnsupportedDevice { ip, model });
    }

    let (children, child_error) = fetch_children(&device, &ip).await;

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

    let mut get_capabilities = vec![GetCapability::DeviceInfo];
    if matches!(device, DiscoveryResult::CameraPtz { .. }) {
        get_capabilities.push(GetCapability::Snapshot);
    }

    tracing::debug!(name, model, ip, "Found device");
    DeviceOutcome::Device {
        device: Device {
            id,
            name,
            model,
            ip,
            set_capabilities,
            get_capabilities,
            children,
        },
        child_error,
    }
}

async fn fetch_children(
    device: &DiscoveryResult,
    ip: &str,
) -> (Vec<ChildDevice>, Option<DiscoveryError>) {
    let result = match device {
        DiscoveryResult::PowerStrip { handler, .. } => {
            handler.get_child_device_list().await.map(|list| {
                list.into_iter()
                    .map(|c| ChildDevice {
                        id: c.device_id,
                        name: c.nickname,
                        model: c.model,
                        set_capabilities: vec![SetCapability::OnOff],
                        get_capabilities: vec![GetCapability::DeviceInfo],
                    })
                    .collect()
            })
        }
        DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => {
            handler.get_child_device_list().await.map(|list| {
                list.into_iter()
                    .map(|c| ChildDevice {
                        id: c.device_id,
                        name: c.nickname,
                        model: c.model,
                        set_capabilities: vec![SetCapability::OnOff],
                        get_capabilities: vec![GetCapability::DeviceInfo],
                    })
                    .collect()
            })
        }
        DiscoveryResult::Hub { handler, .. } => handler
            .get_child_device_list()
            .await
            .map(|list| list.into_iter().map(hub_child_to_child_device).collect()),
        _ => return (vec![], None),
    };

    match result {
        Ok(list) => (list, None),
        Err(err) => {
            tracing::warn!(%err, ip = %ip, "Failed to get child device list");
            (
                vec![],
                Some(DiscoveryError {
                    ip: ip.to_string(),
                    message: format!("child device discovery failed: {err}"),
                }),
            )
        }
    }
}

fn hub_child_to_child_device(child: ChildDeviceHubResult) -> ChildDevice {
    let mut get_capabilities = vec![GetCapability::DeviceInfo];
    match &child {
        ChildDeviceHubResult::S200(_)
        | ChildDeviceHubResult::T100(_)
        | ChildDeviceHubResult::T110(_)
        | ChildDeviceHubResult::T300(_) => get_capabilities.push(GetCapability::TriggerLogs),
        ChildDeviceHubResult::T31X(_) => {
            get_capabilities.push(GetCapability::TemperatureHumidityRecords)
        }
        _ => {}
    }
    ChildDevice {
        id: child.device_id().to_string(),
        name: child.nickname().to_string(),
        model: child.model().to_string(),
        set_capabilities: vec![],
        get_capabilities,
    }
}
