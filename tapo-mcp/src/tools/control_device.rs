use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use tapo::DiscoveryResult;
use tapo::requests::Color;
use tracing::field::Empty;

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{ControlDeviceParams, SetCapabilityRequest};
use crate::requests;
use crate::requests::CheckedDevice;
use crate::telemetry::run_in_span;

pub async fn control_device(
    config: &AppConfig,
    params: ControlDeviceParams,
) -> Result<CallToolResult, McpError> {
    let checked = requests::check_device(config, &params.id, &params.ip).await?;

    for capability in params.capabilities {
        let span = tracing::info_span!(
            "set_capability",
            tapo.capability = capability.name(),
            otel.status_code = Empty,
            "error.type" = Empty,
        );
        run_in_span(span, async {
            match capability {
                SetCapabilityRequest::Brightness { value } => {
                    apply_brightness(&params.id, &checked, value).await
                }
                SetCapabilityRequest::Color { value } => {
                    apply_color(&params.id, &checked, value).await
                }
                SetCapabilityRequest::OnOff { value } => {
                    apply_on_off(&params.id, &checked, value).await
                }
            }
        })
        .await?;
    }

    Ok(CallToolResult::success(vec![]))
}

async fn apply_brightness(
    id: &str,
    checked: &CheckedDevice,
    brightness: u8,
) -> Result<(), TapoMcpError> {
    match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::Light { handler, .. } => handler.set_brightness(brightness).await?,
            DiscoveryResult::ColorLight { handler, .. } => {
                handler.set_brightness(brightness).await?
            }
            DiscoveryResult::RgbLightStrip { handler, .. } => {
                handler.set_brightness(brightness).await?
            }
            DiscoveryResult::RgbicLightStrip { handler, .. } => {
                handler.set_brightness(brightness).await?
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
    }

    Ok(())
}

async fn apply_color(id: &str, checked: &CheckedDevice, color: Color) -> Result<(), TapoMcpError> {
    match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::ColorLight { handler, .. } => handler.set_color(color).await?,
            DiscoveryResult::RgbLightStrip { handler, .. } => handler.set_color(color).await?,
            DiscoveryResult::RgbicLightStrip { handler, .. } => handler.set_color(color).await?,
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
    }

    Ok(())
}

async fn apply_on_off(id: &str, checked: &CheckedDevice, on: bool) -> Result<(), TapoMcpError> {
    macro_rules! on_off {
        ($handler:expr) => {
            if on {
                $handler.on().await?
            } else {
                $handler.off().await?
            }
        };
    }

    match checked {
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
            on_off!(plug);
        }
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            on_off!(plug);
        }
        CheckedDevice::HubChild { .. } => {
            return Err(TapoMcpError::WrongDeviceType {
                id: id.to_string(),
                capability: "OnOff".to_string(),
                expected: "a power strip child".to_string(),
            });
        }
    }

    Ok(())
}
