use std::time::Duration;

use tapo::{
    ApiClient, CameraHubHandler, DiscoveryResult, HubHandler, HubIrHandler,
    PowerStripEnergyMonitoringHandler, PowerStripHandler, StreamExt as _,
    responses::ChildDeviceHubIrResult,
};
use tokio::time::timeout;
use tracing::field::Empty;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
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

/// How long a tool call may contact a device for, on top of the discovery timeout,
/// which is how long finding the device waits when nothing answers at its IP.
const DEVICE_CONTACT_TIMEOUT: Duration = Duration::from_secs(60);

/// Takes the lock on `ip`, finds the device `id` there and passes it to `contact`.
///
/// Gives up once the device has been contacted for longer than the discovery
/// timeout plus [`DEVICE_CONTACT_TIMEOUT`], so that a device that stops responding
/// does not hold the lock on `ip`, and through it every tool call, forever.
pub async fn with_device<T>(
    config: &AppConfig,
    locks: &DeviceLocks,
    id: &str,
    ip: &str,
    contact: impl AsyncFnOnce(CheckedDevice) -> Result<T, TapoMcpError>,
) -> Result<T, TapoMcpError> {
    let _guard = locks.lock_device(ip).await;
    let limit = Duration::from_secs(config.discovery_timeout) + DEVICE_CONTACT_TIMEOUT;
    let span = tracing::info_span!(
        "check_device",
        otel.status_code = Empty,
        "error.type" = Empty,
    );
    // On timeout the tool's device future is dropped here, before `_guard`. A login
    // that discovery has already spawned keeps running until its request ends.
    timeout(limit, async {
        let checked = run_in_span(span, find_device(config, id, ip)).await?;
        contact(checked).await
    })
    .await
    .map_err(|_| TapoMcpError::DeviceTimeout {
        id: id.to_string(),
        ip: ip.to_string(),
        seconds: limit.as_secs(),
    })?
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
