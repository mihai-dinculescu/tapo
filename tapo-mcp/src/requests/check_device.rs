use tapo::{
    ApiClient, CameraHubHandler, DiscoveryResult, HubHandler, HubIrHandler,
    PowerStripEnergyMonitoringHandler, PowerStripHandler, StreamExt as _,
    responses::ChildDeviceHubIrResult,
};

use tracing::field::Empty;

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::telemetry::run_in_span;

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
        hub: HubParent,
        child_id: String,
        /// Hub children as the superset of all the hub child types, so that the
        /// tools can handle them in one place regardless of the hub model.
        child: ChildDeviceHubIrResult,
    },
}

/// The hub a [`CheckedDevice::HubChild`] is paired to.
pub enum HubParent {
    Hub(HubHandler),
    HubIr(HubIrHandler),
    CameraHub(CameraHubHandler),
}

pub async fn check_device(
    config: &AppConfig,
    id: &str,
    ip: &str,
) -> Result<CheckedDevice, TapoMcpError> {
    let span = tracing::info_span!(
        "check_device",
        otel.status_code = Empty,
        "error.type" = Empty,
    );
    run_in_span(span, find_device(config, id, ip)).await
}

async fn find_device(
    config: &AppConfig,
    id: &str,
    ip: &str,
) -> Result<CheckedDevice, TapoMcpError> {
    let api_client = ApiClient::new(config.username.clone(), config.password.clone());
    let mut discovery = api_client
        .discover_devices(ip, config.discovery_timeout)
        .await?;

    let mut last_seen: Option<String> = None;

    while let Some(discovery_result) = discovery.next().await {
        let device = discovery_result.map_err(TapoMcpError::InternalDiscovery)?;
        if device.address().0 != ip {
            continue;
        }

        if device.device_id() == id {
            return Ok(CheckedDevice::Parent(device));
        }

        let device_id = last_seen.is_none().then(|| device.device_id().to_string());

        if let Some(checked) = find_child(device, id).await? {
            return Ok(checked);
        }

        last_seen = last_seen.or(device_id);
    }

    match last_seen {
        Some(found_id) => Err(TapoMcpError::DeviceMismatch {
            expected_id: id.to_string(),
            expected_ip: ip.to_string(),
            found_id,
            found_ip: ip.to_string(),
        }),
        None => Err(TapoMcpError::DeviceNotFound {
            id: id.to_string(),
            ip: ip.to_string(),
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
                hub: HubParent::Hub(handler),
                child_id: c.device_id().to_string(),
                child: c.into(),
            })),
        DiscoveryResult::HubIr { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id() == target_id)
            .map(|c| CheckedDevice::HubChild {
                hub: HubParent::HubIr(handler),
                child_id: c.device_id().to_string(),
                child: c,
            })),
        DiscoveryResult::CameraHub { handler, .. } => Ok(handler
            .get_child_device_list()
            .await?
            .into_iter()
            .find(|c| c.device_id() == target_id)
            .map(|c| CheckedDevice::HubChild {
                hub: HubParent::CameraHub(handler),
                child_id: c.device_id().to_string(),
                child: c.into(),
            })),
        _ => Ok(None),
    }
}
