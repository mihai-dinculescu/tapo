use rmcp::model::{CallToolResult, Content};
use tapo::requests::Color;
use tapo::{DiscoveryResult, Plug};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{CheckDeviceParams, ControlDeviceParams, SetCapabilityRequest};
use crate::requests;
use crate::requests::CheckedDevice;

pub async fn control_device(
    config: &AppConfig,
    params: ControlDeviceParams,
) -> Result<CallToolResult, TapoMcpError> {
    let check_params = CheckDeviceParams {
        id: params.id.clone(),
        ip: params.ip.clone(),
    };
    let checked = requests::check_device(config, check_params).await?;

    for capability in &params.capabilities {
        match capability {
            SetCapabilityRequest::Brightness { value } => {
                apply_brightness(&params.id, &checked, *value).await?
            }
            SetCapabilityRequest::Color { value } => {
                apply_color(&params.id, &checked, value.clone()).await?
            }
            SetCapabilityRequest::OnOff { value } => {
                apply_on_off(&params.id, &checked, *value).await?
            }
        }
    }

    let applied: Vec<_> = params
        .capabilities
        .iter()
        .map(ToString::to_string)
        .collect();
    let content = vec![Content::text(format!(
        "Device {} applied: {}.",
        params.id,
        applied.join(", "),
    ))];
    Ok(CallToolResult::success(content))
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
                return Err(TapoMcpError::UnsupportedCapability {
                    id: id.to_string(),
                    capability: "Brightness".to_string(),
                });
            }
        },
        CheckedDevice::Child { .. } => {
            return Err(TapoMcpError::UnsupportedCapability {
                id: id.to_string(),
                capability: "Brightness".to_string(),
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
                return Err(TapoMcpError::UnsupportedCapability {
                    id: id.to_string(),
                    capability: "Color".to_string(),
                });
            }
        },
        CheckedDevice::Child { .. } => {
            return Err(TapoMcpError::UnsupportedCapability {
                id: id.to_string(),
                capability: "Color".to_string(),
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
            DiscoveryResult::GenericDevice { .. }
            | DiscoveryResult::PowerStrip { .. }
            | DiscoveryResult::PowerStripEnergyMonitoring { .. }
            | DiscoveryResult::Hub { .. } => {
                return Err(TapoMcpError::UnsupportedCapability {
                    id: id.to_string(),
                    capability: "OnOff".to_string(),
                });
            }
        },
        CheckedDevice::Child { parent, child_id } => match parent {
            DiscoveryResult::PowerStrip { handler, .. } => {
                let plug = handler.plug(Plug::ByDeviceId(child_id.clone())).await?;
                on_off!(plug);
            }
            DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => {
                let plug = handler.plug(Plug::ByDeviceId(child_id.clone())).await?;
                on_off!(plug);
            }
            _ => {
                return Err(TapoMcpError::UnsupportedCapability {
                    id: id.to_string(),
                    capability: "OnOff".to_string(),
                });
            }
        },
    }

    Ok(())
}
