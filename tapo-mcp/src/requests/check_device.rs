use tapo::{ApiClient, DiscoveryResult, StreamExt as _};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::CheckDeviceParams;

pub enum CheckedDevice {
    Parent(DiscoveryResult),
    Child {
        parent: DiscoveryResult,
        child_id: String,
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
            && let Some(child_id) = find_child_id(&device, &params.id).await?
        {
            return Ok(CheckedDevice::Child {
                parent: device,
                child_id,
            });
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

async fn find_child_id(
    device: &DiscoveryResult,
    target_id: &str,
) -> Result<Option<String>, TapoMcpError> {
    let child_ids = match device {
        DiscoveryResult::PowerStrip { handler, .. } => Some(
            handler
                .get_child_device_list()
                .await?
                .into_iter()
                .map(|c| c.device_id)
                .collect::<Vec<_>>(),
        ),
        DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => Some(
            handler
                .get_child_device_list()
                .await?
                .into_iter()
                .map(|c| c.device_id)
                .collect::<Vec<_>>(),
        ),
        _ => None,
    };

    Ok(child_ids.and_then(|ids| ids.into_iter().find(|id| id == target_id)))
}
