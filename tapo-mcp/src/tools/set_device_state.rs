use rmcp::model::{CallToolResult, Content};
use tapo::{DiscoveryResult, Plug};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{CheckDeviceParams, SetCapabilityRequest, SetDeviceStateParams};
use crate::requests;
use crate::requests::CheckedDevice;

pub async fn set_device_state(
    config: &AppConfig,
    params: SetDeviceStateParams,
) -> Result<CallToolResult, TapoMcpError> {
    let check_params = CheckDeviceParams {
        id: params.id.clone(),
        ip: params.ip.clone(),
    };
    let checked = requests::check_device(config, check_params).await?;

    match &params.capability {
        SetCapabilityRequest::OnOff(on) => apply_on_off(&params.id, checked, *on).await?,
    }

    let content = vec![Content::text(format!(
        "Device {id} {capability:?} applied.",
        id = params.id,
        capability = params.capability,
    ))];
    Ok(CallToolResult::success(content))
}

async fn apply_on_off(id: &str, checked: CheckedDevice, on: bool) -> Result<(), TapoMcpError> {
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
                let plug = handler.plug(Plug::ByDeviceId(child_id)).await?;
                on_off!(plug);
            }
            DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => {
                let plug = handler.plug(Plug::ByDeviceId(child_id)).await?;
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
