use std::fmt::Display;
use std::ops::RangeInclusive;
use std::pin::Pin;

use rmcp::model::{CallToolResult, ContentBlock};
use tapo::DiscoveryResult;
use tapo::requests::{
    BRIGHTNESS_RANGE, COLOR_TEMPERATURE_RANGE, Color, HUE_RANGE, SATURATION_RANGE,
};
use tapo::responses::ChildDeviceHubIrResult;
use tracing::field::Empty;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
use crate::errors::TapoMcpError;
use crate::models::{
    CONTROL_CAPABILITIES, ControlDeviceParams, ControlDeviceResult, GetCapability,
    SetCapabilityRequest,
};
use crate::requests;
use crate::requests::CheckedDevice;
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

    validate(&params.capabilities)?;

    // Counted outside the device future, which a timeout drops partway through.
    let mut applied = 0;
    let mut in_flight = None;
    let device_info =
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
                        SetCapabilityRequest::Color { value } => color_request(
                            &params.id,
                            &checked,
                            name,
                            ColorOperation::Preset(value),
                        )?,
                        SetCapabilityRequest::HueSaturation { hue, saturation } => color_request(
                            &params.id,
                            &checked,
                            name,
                            ColorOperation::HueSaturation(hue, saturation),
                        )?,
                        SetCapabilityRequest::ColorTemperature { value } => color_request(
                            &params.id,
                            &checked,
                            name,
                            ColorOperation::ColorTemperature(value),
                        )?,
                        SetCapabilityRequest::OnOff { value } => {
                            on_off_request(&params.id, &checked, value)?
                        }
                    };
                    Ok((name, request))
                })
                .collect::<Result<Vec<_>, TapoMcpError>>()?;
            // Built before anything is applied too, so a device whose state can't
            // be read back also fails the call with nothing changed.
            let read_back = device_info_request(&params.id, &checked)?;

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

            let span = tracing::info_span!(
                "get_capability",
                tapo.capability = GetCapability::DeviceInfo.name(),
                otel.status_code = Empty,
                "error.type" = Empty,
            );
            // Every failure here is reported as a failed read-back, which tells
            // the caller the capabilities were applied.
            run_in_span(span, async {
                read_back
                    .await
                    .map_err(|source| TapoMcpError::ControlReadBackFailed {
                        total: count,
                        source,
                    })
            })
            .await
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

    let result = ControlDeviceResult {
        id: params.id,
        ip: params.ip,
        device_info,
    };
    let content = vec![ContentBlock::text(serde_json::to_string(&result)?)];
    Ok(CallToolResult::success(content))
}

/// The request that reads the device's info back over the session the set
/// capabilities are applied on, so that the result shows the state they produced.
fn device_info_request<'a>(
    id: &str,
    checked: &'a CheckedDevice,
) -> Result<DeviceInfoRequest<'a>, TapoMcpError> {
    macro_rules! read {
        ($handler:expr) => {
            Box::pin(async move {
                let info = $handler.get_device_info().await?;
                Ok::<_, tapo::Error>(serde_json::to_value(&info)?)
            })
        };
    }

    let wrong_device_type = || TapoMcpError::WrongDeviceType {
        id: id.to_string(),
        capability: GetCapability::DeviceInfo.name().to_string(),
        expected: "a light, a plug, a power strip child or an S210 switch".to_string(),
    };

    let request: DeviceInfoRequest<'a> = match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::Light { handler, .. } => read!(handler),
            DiscoveryResult::ColorLight { handler, .. } => read!(handler),
            DiscoveryResult::RgbLightStrip { handler, .. } => read!(handler),
            DiscoveryResult::RgbicLightStrip { handler, .. } => read!(handler),
            DiscoveryResult::Plug { handler, .. } => read!(handler),
            DiscoveryResult::PlugEnergyMonitoring { handler, .. } => read!(handler),
            DiscoveryResult::Other { .. }
            | DiscoveryResult::PowerStrip { .. }
            | DiscoveryResult::PowerStripEnergyMonitoring { .. }
            | DiscoveryResult::Hub { .. }
            | DiscoveryResult::HubIr { .. }
            | DiscoveryResult::CameraHub { .. }
            | DiscoveryResult::Camera { .. }
            | DiscoveryResult::CameraPtz { .. } => return Err(wrong_device_type()),
            // `DiscoveryResult` is non-exhaustive. Every type it has today is handled above.
            _ => unreachable!(),
        },
        CheckedDevice::PowerStripChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            read!(plug)
        }
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            read!(plug)
        }
        CheckedDevice::HubChild {
            hub,
            child_id,
            child: ChildDeviceHubIrResult::S210(_),
        } => {
            let switch = hub.s210_unchecked(child_id);
            read!(switch)
        }
        CheckedDevice::HubChild { .. } => return Err(wrong_device_type()),
    };

    Ok(request)
}

/// Checks every capability's value before any capability is applied. The device
/// handlers check them too, but only when the request is sent.
fn validate(capabilities: &[SetCapabilityRequest]) -> Result<(), TapoMcpError> {
    for (index, capability) in capabilities.iter().enumerate() {
        let position = index + 1;
        let name = capability.name();
        match capability {
            SetCapabilityRequest::Brightness { value } => {
                check_range(position, name, None, &BRIGHTNESS_RANGE, *value)?;
            }
            SetCapabilityRequest::HueSaturation { hue, saturation } => {
                check_range(position, name, Some("hue"), &HUE_RANGE, *hue)?;
                check_range(
                    position,
                    name,
                    Some("saturation"),
                    &SATURATION_RANGE,
                    *saturation,
                )?;
            }
            SetCapabilityRequest::ColorTemperature { value } => {
                check_range(position, name, None, &COLOR_TEMPERATURE_RANGE, *value)?;
            }
            SetCapabilityRequest::Color { .. } | SetCapabilityRequest::OnOff { .. } => {}
        }
    }

    Ok(())
}

fn check_range<T: PartialOrd + Display>(
    position: usize,
    capability: &str,
    field: Option<&str>,
    range: &RangeInclusive<T>,
    value: T,
) -> Result<(), TapoMcpError> {
    if range.contains(&value) {
        return Ok(());
    }

    let field = field.map(|field| format!(" {field}")).unwrap_or_default();
    Err(TapoMcpError::InvalidParameter {
        name: "capabilities".to_string(),
        message: format!(
            "capability {position} ({capability}){field} must be between {} and {}, got {value}",
            range.start(),
            range.end()
        ),
    })
}

/// A capability's request to the device, sent when awaited.
type DeviceRequest<'a> = Pin<Box<dyn Future<Output = Result<(), tapo::Error>> + Send + 'a>>;

/// The request that reads the device's info, sent when awaited.
type DeviceInfoRequest<'a> =
    Pin<Box<dyn Future<Output = Result<serde_json::Value, tapo::Error>> + Send + 'a>>;

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

/// A color change, sent to every color light the same way.
enum ColorOperation {
    Preset(Color),
    HueSaturation(u16, u8),
    ColorTemperature(u16),
}

fn color_request<'a>(
    id: &str,
    checked: &'a CheckedDevice,
    capability: &str,
    operation: ColorOperation,
) -> Result<DeviceRequest<'a>, TapoMcpError> {
    macro_rules! color {
        ($handler:expr) => {
            match operation {
                ColorOperation::Preset(color) => Box::pin($handler.set_color(color)),
                ColorOperation::HueSaturation(hue, saturation) => {
                    Box::pin($handler.set_hue_saturation(hue, saturation))
                }
                ColorOperation::ColorTemperature(color_temperature) => {
                    Box::pin($handler.set_color_temperature(color_temperature))
                }
            }
        };
    }

    let request: DeviceRequest<'a> = match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::ColorLight { handler, .. } => color!(handler),
            DiscoveryResult::RgbLightStrip { handler, .. } => color!(handler),
            DiscoveryResult::RgbicLightStrip { handler, .. } => color!(handler),
            _ => {
                return Err(TapoMcpError::WrongDeviceType {
                    id: id.to_string(),
                    capability: capability.to_string(),
                    expected: "a color light device".to_string(),
                });
            }
        },
        CheckedDevice::PowerStripChild { .. }
        | CheckedDevice::PowerStripEnergyMonitoringChild { .. }
        | CheckedDevice::HubChild { .. } => {
            return Err(TapoMcpError::WrongDeviceType {
                id: id.to_string(),
                capability: capability.to_string(),
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
            let switch = hub.s210_unchecked(child_id);
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
    fn validate_brightness_accepts_values_in_range() {
        let capabilities = [
            SetCapabilityRequest::OnOff { value: true },
            SetCapabilityRequest::Brightness { value: 1 },
            SetCapabilityRequest::Brightness { value: 100 },
        ];

        assert!(validate(&capabilities).is_ok());
    }

    #[test]
    fn validate_brightness_rejects_values_out_of_range() {
        for value in [0, 101] {
            let capabilities = [
                SetCapabilityRequest::OnOff { value: true },
                SetCapabilityRequest::Brightness { value },
            ];

            assert!(matches!(
                validate(&capabilities),
                Err(TapoMcpError::InvalidParameter { name, message })
                    if name == "capabilities"
                        && message == format!("capability 2 (Brightness) must be between 1 and 100, got {value}")
            ));
        }
    }

    #[test]
    fn validate_hue_saturation_accepts_values_in_range() {
        let capabilities = [
            SetCapabilityRequest::OnOff { value: true },
            SetCapabilityRequest::HueSaturation {
                hue: 0,
                saturation: 1,
            },
            SetCapabilityRequest::HueSaturation {
                hue: 360,
                saturation: 100,
            },
        ];

        assert!(validate(&capabilities).is_ok());
    }

    #[test]
    fn validate_hue_saturation_rejects_hue_out_of_range() {
        for hue in [361, 362] {
            let capabilities = [SetCapabilityRequest::HueSaturation {
                hue,
                saturation: 50,
            }];

            assert!(matches!(
                validate(&capabilities),
                Err(TapoMcpError::InvalidParameter { name, message })
                    if name == "capabilities"
                        && message == format!("capability 1 (HueSaturation) hue must be between 0 and 360, got {hue}")
            ));
        }
    }

    #[test]
    fn validate_hue_saturation_rejects_saturation_out_of_range() {
        for saturation in [0, 101] {
            let capabilities = [SetCapabilityRequest::HueSaturation {
                hue: 120,
                saturation,
            }];

            assert!(matches!(
                validate(&capabilities),
                Err(TapoMcpError::InvalidParameter { name, message })
                    if name == "capabilities"
                        && message == format!("capability 1 (HueSaturation) saturation must be between 1 and 100, got {saturation}")
            ));
        }
    }

    #[test]
    fn validate_color_temperature_accepts_values_in_range() {
        let capabilities = [
            SetCapabilityRequest::OnOff { value: true },
            SetCapabilityRequest::ColorTemperature { value: 2500 },
            SetCapabilityRequest::ColorTemperature { value: 6500 },
        ];

        assert!(validate(&capabilities).is_ok());
    }

    #[test]
    fn validate_color_temperature_rejects_values_out_of_range() {
        for value in [2499, 6501] {
            let capabilities = [SetCapabilityRequest::ColorTemperature { value }];

            assert!(matches!(
                validate(&capabilities),
                Err(TapoMcpError::InvalidParameter { name, message })
                    if name == "capabilities"
                        && message == format!("capability 1 (ColorTemperature) must be between 2500 and 6500, got {value}")
            ));
        }
    }
}
