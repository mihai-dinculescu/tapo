use std::pin::Pin;

use rmcp::model::CallToolResult;
use tapo::DiscoveryResult;
use tapo::requests::Color;
use tapo::responses::ChildDeviceHubIrResult;
use tracing::field::Empty;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
use crate::errors::TapoMcpError;
use crate::models::{
    BRIGHTNESS_LEVELS, CONTROL_CAPABILITIES, ControlDeviceParams, SetCapabilityRequest,
};
use crate::requests;
use crate::requests::{CheckedDevice, HubParent};
use crate::telemetry::run_in_span;

pub async fn control_device(
    config: &AppConfig,
    locks: &DeviceLocks,
    params: ControlDeviceParams,
) -> Result<CallToolResult, TapoMcpError> {
    let count = params.capabilities.len();
    if !CONTROL_CAPABILITIES.contains(&count) {
        return Err(TapoMcpError::InvalidParameter {
            name: "capabilities".to_string(),
            message: format!(
                "must have between {} and {} entries, got {count}",
                CONTROL_CAPABILITIES.start(),
                CONTROL_CAPABILITIES.end()
            ),
        });
    }

    check_brightness(&params.capabilities)?;

    // Counted outside the device future, which a timeout drops partway through.
    let mut applied = 0;
    let mut in_flight = None;
    requests::with_device(config, locks, &params.id, &params.ip, async |checked| {
        // Every capability is checked against the device before any is applied,
        // so one the device lacks fails the call with nothing changed.
        let requests = params
            .capabilities
            .into_iter()
            .map(|capability| {
                let name = capability.name();
                let request = match capability {
                    SetCapabilityRequest::Brightness { value } => {
                        brightness_request(&params.id, &checked, value)?
                    }
                    SetCapabilityRequest::Color { value } => {
                        color_request(&params.id, &checked, value)?
                    }
                    SetCapabilityRequest::OnOff { value } => {
                        on_off_request(&params.id, &checked, value)?
                    }
                };
                Ok((name, request))
            })
            .collect::<Result<Vec<_>, TapoMcpError>>()?;

        for (name, request) in requests {
            let span = tracing::info_span!(
                "set_capability",
                tapo.capability = name,
                otel.status_code = Empty,
                "error.type" = Empty,
            );
            in_flight = Some(name);
            run_in_span(span, async {
                request.await.map_err(|source| TapoMcpError::ControlFailed {
                    applied,
                    total: count,
                    failed: name,
                    source,
                })
            })
            .await?;
            in_flight = None;
            applied += 1;
        }
        Ok(())
    })
    .await
    .map_err(|err| match err {
        TapoMcpError::DeviceTimeout { id, ip, seconds } => TapoMcpError::ControlTimeout {
            id,
            ip,
            seconds,
            applied,
            total: count,
            in_flight,
        },
        err => err,
    })?;

    Ok(CallToolResult::success(vec![]))
}

/// Checks every `Brightness` value before any capability is applied. The device
/// handlers check it too, but only when the request is sent.
fn check_brightness(capabilities: &[SetCapabilityRequest]) -> Result<(), TapoMcpError> {
    for (index, capability) in capabilities.iter().enumerate() {
        if let SetCapabilityRequest::Brightness { value } = capability
            && !BRIGHTNESS_LEVELS.contains(value)
        {
            return Err(TapoMcpError::InvalidParameter {
                name: "capabilities".to_string(),
                message: format!(
                    "capability {} (Brightness) must be between {} and {}, got {value}",
                    index + 1,
                    BRIGHTNESS_LEVELS.start(),
                    BRIGHTNESS_LEVELS.end()
                ),
            });
        }
    }

    Ok(())
}

/// A capability's request to the device, sent when awaited.
type DeviceRequest<'a> = Pin<Box<dyn Future<Output = Result<(), tapo::Error>> + Send + 'a>>;

fn brightness_request<'a>(
    id: &str,
    checked: &'a CheckedDevice,
    brightness: u8,
) -> Result<DeviceRequest<'a>, TapoMcpError> {
    let request: DeviceRequest<'a> = match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::Light { handler, .. } => Box::pin(handler.set_brightness(brightness)),
            DiscoveryResult::ColorLight { handler, .. } => {
                Box::pin(handler.set_brightness(brightness))
            }
            DiscoveryResult::RgbLightStrip { handler, .. } => {
                Box::pin(handler.set_brightness(brightness))
            }
            DiscoveryResult::RgbicLightStrip { handler, .. } => {
                Box::pin(handler.set_brightness(brightness))
            }
            _ => {
                return Err(TapoMcpError::WrongDeviceType {
                    id: id.to_string(),
                    capability: "Brightness".to_string(),
                    expected: "a light device".to_string(),
                });
            }
        },
        CheckedDevice::PowerStripChild { .. }
        | CheckedDevice::PowerStripEnergyMonitoringChild { .. }
        | CheckedDevice::HubChild { .. } => {
            return Err(TapoMcpError::WrongDeviceType {
                id: id.to_string(),
                capability: "Brightness".to_string(),
                expected: "a light device".to_string(),
            });
        }
    };

    Ok(request)
}

fn color_request<'a>(
    id: &str,
    checked: &'a CheckedDevice,
    color: Color,
) -> Result<DeviceRequest<'a>, TapoMcpError> {
    let request: DeviceRequest<'a> = match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::ColorLight { handler, .. } => Box::pin(handler.set_color(color)),
            DiscoveryResult::RgbLightStrip { handler, .. } => Box::pin(handler.set_color(color)),
            DiscoveryResult::RgbicLightStrip { handler, .. } => Box::pin(handler.set_color(color)),
            _ => {
                return Err(TapoMcpError::WrongDeviceType {
                    id: id.to_string(),
                    capability: "Color".to_string(),
                    expected: "a color light device".to_string(),
                });
            }
        },
        CheckedDevice::PowerStripChild { .. }
        | CheckedDevice::PowerStripEnergyMonitoringChild { .. }
        | CheckedDevice::HubChild { .. } => {
            return Err(TapoMcpError::WrongDeviceType {
                id: id.to_string(),
                capability: "Color".to_string(),
                expected: "a color light device".to_string(),
            });
        }
    };

    Ok(request)
}

fn on_off_request<'a>(
    id: &str,
    checked: &'a CheckedDevice,
    on: bool,
) -> Result<DeviceRequest<'a>, TapoMcpError> {
    macro_rules! on_off {
        ($handler:expr) => {
            Box::pin(async move {
                if on {
                    $handler.on().await
                } else {
                    $handler.off().await
                }
            })
        };
    }

    let request: DeviceRequest<'a> = match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::Light { handler, .. } => on_off!(handler),
            DiscoveryResult::ColorLight { handler, .. } => on_off!(handler),
            DiscoveryResult::RgbLightStrip { handler, .. } => on_off!(handler),
            DiscoveryResult::RgbicLightStrip { handler, .. } => on_off!(handler),
            DiscoveryResult::Plug { handler, .. } => on_off!(handler),
            DiscoveryResult::PlugEnergyMonitoring { handler, .. } => on_off!(handler),
            DiscoveryResult::Other { .. }
            | DiscoveryResult::PowerStrip { .. }
            | DiscoveryResult::PowerStripEnergyMonitoring { .. }
            | DiscoveryResult::Hub { .. }
            | DiscoveryResult::HubIr { .. }
            | DiscoveryResult::CameraHub { .. }
            | DiscoveryResult::Camera { .. }
            | DiscoveryResult::CameraPtz { .. } => {
                return Err(TapoMcpError::WrongDeviceType {
                    id: id.to_string(),
                    capability: "OnOff".to_string(),
                    expected: "an on/off-capable device".to_string(),
                });
            }
            // `DiscoveryResult` is non-exhaustive. Every type it has today is handled above.
            _ => unreachable!(),
        },
        CheckedDevice::PowerStripChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            on_off!(plug)
        }
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            on_off!(plug)
        }
        CheckedDevice::HubChild {
            hub,
            child_id,
            child: ChildDeviceHubIrResult::S210(_),
        } => {
            let switch = match hub {
                HubParent::Hub(hub) => hub.s210_unchecked(child_id.clone()),
                HubParent::HubIr(hub) => hub.s210_unchecked(child_id.clone()),
                HubParent::CameraHub(hub) => hub.s210_unchecked(child_id.clone()),
            };
            on_off!(switch)
        }
        CheckedDevice::HubChild { .. } => {
            return Err(TapoMcpError::WrongDeviceType {
                id: id.to_string(),
                capability: "OnOff".to_string(),
                expected: "a power strip child or an S210 switch".to_string(),
            });
        }
    };

    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_brightness_accepts_values_in_range() {
        let capabilities = [
            SetCapabilityRequest::OnOff { value: true },
            SetCapabilityRequest::Brightness { value: 1 },
            SetCapabilityRequest::Brightness { value: 100 },
        ];

        assert!(check_brightness(&capabilities).is_ok());
    }

    #[test]
    fn check_brightness_rejects_values_out_of_range() {
        for value in [0, 101] {
            let capabilities = [
                SetCapabilityRequest::OnOff { value: true },
                SetCapabilityRequest::Brightness { value },
            ];

            assert!(matches!(
                check_brightness(&capabilities),
                Err(TapoMcpError::InvalidParameter { name, message })
                    if name == "capabilities"
                        && message == format!("capability 2 (Brightness) must be between 1 and 100, got {value}")
            ));
        }
    }
}
