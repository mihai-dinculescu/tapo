use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};
use tapo::{DiscoveryResult, Plug};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{CheckDeviceParams, GetCapabilityRequest, GetDeviceStateParams};
use crate::requests;
use crate::requests::CheckedDevice;

pub async fn get_device_state(
    config: &AppConfig,
    params: GetDeviceStateParams,
) -> Result<CallToolResult, McpError> {
    let check_params = CheckDeviceParams {
        id: params.id.clone(),
        ip: params.ip.clone(),
    };
    let checked = requests::check_device(config, check_params).await?;

    let value = match &params.capability {
        GetCapabilityRequest::DeviceInfo => get_device_info(&params.id, checked).await?,
    };

    let content = vec![Content::json(value)?];
    Ok(CallToolResult::success(content))
}

async fn get_device_info(
    id: &str,
    checked: CheckedDevice,
) -> Result<serde_json::Value, TapoMcpError> {
    match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::GenericDevice { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Light { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
            DiscoveryResult::ColorLight { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::RgbLightStrip { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::RgbicLightStrip { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Plug { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
            DiscoveryResult::PlugEnergyMonitoring { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::PowerStrip { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Hub { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
        },
        CheckedDevice::Child { parent, child_id } => match parent {
            DiscoveryResult::PowerStrip { handler, .. } => {
                let plug = handler.plug(Plug::ByDeviceId(child_id)).await?;
                let info = plug.get_device_info().await?;
                Ok(serde_json::to_value(&info)?)
            }
            DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => {
                let plug = handler.plug(Plug::ByDeviceId(child_id)).await?;
                let info = plug.get_device_info().await?;
                Ok(serde_json::to_value(&info)?)
            }
            _ => Err(TapoMcpError::UnsupportedCapability {
                id: id.to_string(),
                capability: "DeviceInfo".to_string(),
            }),
        },
    }
}
