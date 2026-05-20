use tapo::{
    ApiClient, DiscoveryResult, HubHandler, PowerStripEnergyMonitoringHandler, PowerStripHandler,
    StreamExt as _, responses::ChildDeviceHubResult,
};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::CheckDeviceParams;

pub enum CheckedDevice {
    Parent(DiscoveryResult),
    PowerStripChild {
        handler: PowerStripHandler,
        child_id: String,
    },
    PowerStripEnergyMonitoringChild {
        handler: PowerStripEnergyMonitoringHandler,
        child_id: String,
    },
    HubChild {
        handler: HubHandler,
        child_id: String,
        child: ChildDeviceHubResult,
    },
}

pub async fn check_device(
    config: &AppConfig,
    params: CheckDeviceParams,
) -> Result<CheckedDevice, TapoMcpError> {
    let api_client = ApiClient::new(config.username.clone(), config.password.clone());
    let mut discovery = api_client
        .discover_devices(params.ip.clone(), config.discovery_timeout)
        .await?;

    let mut last_seen: Option<(String, String)> = None;

    while let Some(discovery_result) = discovery.next().await {
        let device = discovery_result.map_err(TapoMcpError::InternalDiscovery)?;
        let device_id = device.device_id().to_string();
        let device_ip = device.ip().to_string();

        if device_ip == params.ip && device_id == params.id {
            return Ok(CheckedDevice::Parent(device));
        }

        if device_ip == params.ip
            && let Some(checked) = find_child(device, &params.id).await?
        {
            return Ok(checked);
        }

        if device_ip == params.ip {
            last_seen.get_or_insert((device_id, device_ip));
        }
    }

    match last_seen {
        Some((found_id, found_ip)) => Err(TapoMcpError::DeviceMismatch {
            expected_id: params.id,
            expected_ip: params.ip,
            found_id,
            found_ip,
        }),
        None => Err(TapoMcpError::DeviceNotFound {
            id: params.id,
            ip: params.ip,
        }),
    }
}

async fn find_child(
    device: DiscoveryResult,
    target_id: &str,
) -> Result<Option<CheckedDevice>, TapoMcpError> {
    match device {
        DiscoveryResult::PowerStrip { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id == target_id)
            .map(|c| CheckedDevice::PowerStripChild {
                handler,
                child_id: c.device_id,
            })),
        DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id == target_id)
            .map(|c| CheckedDevice::PowerStripEnergyMonitoringChild {
                handler,
                child_id: c.device_id,
            })),
        DiscoveryResult::Hub { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id() == target_id)
            .map(|c| CheckedDevice::HubChild {
                handler,
                child_id: c.device_id().to_string(),
                child: c,
            })),
        _ => Ok(None),
    }
}
