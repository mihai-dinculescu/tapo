use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};
use tapo::DiscoveryResult;
use tapo::responses::ChildDeviceHubResult;

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
        GetCapabilityRequest::DeviceInfo => get_device_info(checked).await?,
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

async fn get_device_info(checked: CheckedDevice) -> Result<serde_json::Value, TapoMcpError> {
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
        CheckedDevice::PowerStripChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            let info = plug.get_device_info().await?;
            Ok(serde_json::to_value(&info)?)
        }
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            let info = plug.get_device_info().await?;
            Ok(serde_json::to_value(&info)?)
        }
        CheckedDevice::HubChild { child, .. } => Ok(serde_json::to_value(&child)?),
    }
}

async fn get_trigger_logs(
    id: &str,
    checked: CheckedDevice,
    page_size: u64,
    start_id: u64,
) -> Result<serde_json::Value, TapoMcpError> {
    let CheckedDevice::HubChild {
        handler,
        child_id,
        child,
    } = checked
    else {
        return Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: "TriggerLogs".to_string(),
            expected: "a hub child device".to_string(),
        });
    };

    macro_rules! trigger_logs {
        ($constructor:ident) => {{
            let h = handler.$constructor(child_id);
            Ok(serde_json::to_value(
                &h.get_trigger_logs(page_size, start_id).await?,
            )?)
        }};
    }

    match child {
        ChildDeviceHubResult::S200(_) => trigger_logs!(s200_unchecked),
        ChildDeviceHubResult::T100(_) => trigger_logs!(t100_unchecked),
        ChildDeviceHubResult::T110(_) => trigger_logs!(t110_unchecked),
        ChildDeviceHubResult::T300(_) => trigger_logs!(t300_unchecked),
        _ => Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: "TriggerLogs".to_string(),
            expected: "a trigger-based sensor (S200B/D, T100, T110, T300)".to_string(),
        }),
    }
}

async fn get_temperature_humidity_records(
    id: &str,
    checked: CheckedDevice,
) -> Result<serde_json::Value, TapoMcpError> {
    let CheckedDevice::HubChild {
        handler,
        child_id,
        child,
    } = checked
    else {
        return Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: "TemperatureHumidityRecords".to_string(),
            expected: "a hub child device".to_string(),
        });
    };

    match child {
        ChildDeviceHubResult::T31X(_) => {
            let h = handler.t31x_unchecked(child_id);
            Ok(serde_json::to_value(
                &h.get_temperature_humidity_records().await?,
            )?)
        }
        _ => Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: "TemperatureHumidityRecords".to_string(),
            expected: "a T310 or T315 sensor".to_string(),
        }),
    }
}
