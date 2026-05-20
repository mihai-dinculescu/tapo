use tapo::responses::ChildDeviceHubResult;
use tapo::{ApiClient, DiscoveryResult, StreamExt as _};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::CheckDeviceParams;

pub enum CheckedDevice {
    Parent(DiscoveryResult),
    Child {
        parent: DiscoveryResult,
        child_id: String,
        /// The matched hub child variant, populated when `parent` is `DiscoveryResult::Hub`.
        hub_child: Option<ChildDeviceHubResult>,
    },
}

struct MatchedChild {
    id: String,
    hub_child: Option<ChildDeviceHubResult>,
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
            && let Some(matched) = find_child(&device, &params.id).await?
        {
            return Ok(CheckedDevice::Child {
                parent: device,
                child_id: matched.id,
                hub_child: matched.hub_child,
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

async fn find_child(
    device: &DiscoveryResult,
    target_id: &str,
) -> Result<Option<MatchedChild>, TapoMcpError> {
    match device {
        DiscoveryResult::PowerStrip { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id == target_id)
            .map(|c| MatchedChild {
                id: c.device_id,
                hub_child: None,
            })),
        DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id == target_id)
            .map(|c| MatchedChild {
                id: c.device_id,
                hub_child: None,
            })),
        DiscoveryResult::Hub { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id() == target_id)
            .map(|c| MatchedChild {
                id: c.device_id().to_string(),
                hub_child: Some(c),
            })),
        _ => Ok(None),
    }
}
