use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};
use tapo::responses::ChildDeviceHubResult;
use tapo::{DiscoveryResult, HubHandler};

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

    let value = match params.capability {
        GetCapabilityRequest::DeviceInfo => get_device_info(&params.id, checked).await?,
        GetCapabilityRequest::Snapshot => {
            return Err(TapoMcpError::WrongTool {
                capability: "Snapshot".to_string(),
                tool: "take_snapshot".to_string(),
            }
            .into());
        }
        GetCapabilityRequest::TriggerLogs {
            page_size,
            start_id,
        } => get_trigger_logs(&params.id, checked, page_size, start_id).await?,
        GetCapabilityRequest::TemperatureHumidityRecords => {
            get_temperature_humidity_records(&params.id, checked).await?
        }
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
            DiscoveryResult::CameraPtz { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Other { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
        },
        CheckedDevice::Child {
            parent,
            child_id,
            hub_child,
        } => match parent {
            DiscoveryResult::PowerStrip { handler, .. } => {
                let plug = handler.plug_unchecked(child_id);
                let info = plug.get_device_info().await?;
                Ok(serde_json::to_value(&info)?)
            }
            DiscoveryResult::PowerStripEnergyMonitoring { handler, .. } => {
                let plug = handler.plug_unchecked(child_id);
                let info = plug.get_device_info().await?;
                Ok(serde_json::to_value(&info)?)
            }
            DiscoveryResult::Hub { .. } => match hub_child {
                Some(child) => Ok(serde_json::to_value(&child)?),
                None => Err(TapoMcpError::UnsupportedCapability {
                    id: id.to_string(),
                    capability: "DeviceInfo".to_string(),
                }),
            },
            _ => Err(TapoMcpError::UnsupportedCapability {
                id: id.to_string(),
                capability: "DeviceInfo".to_string(),
            }),
        },
    }
}

async fn get_trigger_logs(
    id: &str,
    checked: CheckedDevice,
    page_size: u64,
    start_id: u64,
) -> Result<serde_json::Value, TapoMcpError> {
    let (handler, child_id, hub_child) = require_hub_child(id, checked, "TriggerLogs")?;

    macro_rules! trigger_logs {
        ($constructor:ident) => {{
            let h = handler.$constructor(child_id);
            Ok(serde_json::to_value(
                &h.get_trigger_logs(page_size, start_id).await?,
            )?)
        }};
    }

    match hub_child {
        ChildDeviceHubResult::S200(_) => trigger_logs!(s200_unchecked),
        ChildDeviceHubResult::T100(_) => trigger_logs!(t100_unchecked),
        ChildDeviceHubResult::T110(_) => trigger_logs!(t110_unchecked),
        ChildDeviceHubResult::T300(_) => trigger_logs!(t300_unchecked),
        _ => Err(TapoMcpError::UnsupportedCapability {
            id: id.to_string(),
            capability: "TriggerLogs".to_string(),
        }),
    }
}

async fn get_temperature_humidity_records(
    id: &str,
    checked: CheckedDevice,
) -> Result<serde_json::Value, TapoMcpError> {
    let (handler, child_id, hub_child) =
        require_hub_child(id, checked, "TemperatureHumidityRecords")?;
    match hub_child {
        ChildDeviceHubResult::T31X(_) => {
            let h = handler.t31x_unchecked(child_id);
            Ok(serde_json::to_value(
                &h.get_temperature_humidity_records().await?,
            )?)
        }
        _ => Err(TapoMcpError::UnsupportedCapability {
            id: id.to_string(),
            capability: "TemperatureHumidityRecords".to_string(),
        }),
    }
}

fn require_hub_child(
    id: &str,
    checked: CheckedDevice,
    capability: &str,
) -> Result<(HubHandler, String, ChildDeviceHubResult), TapoMcpError> {
    match checked {
        CheckedDevice::Child {
            parent: DiscoveryResult::Hub { handler, .. },
            child_id,
            hub_child: Some(hub_child),
        } => Ok((handler, child_id, hub_child)),
        _ => Err(TapoMcpError::UnsupportedCapability {
            id: id.to_string(),
            capability: capability.to_string(),
        }),
    }
}
