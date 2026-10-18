use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
use crate::models::CheckDeviceParams;
use crate::requests;

pub async fn check_device(
    config: &AppConfig,
    locks: &DeviceLocks,
    params: CheckDeviceParams,
) -> Result<CallToolResult, McpError> {
    requests::check_device(config, locks, &params.id, &params.ip).await?;
    Ok(CallToolResult::success(vec![]))
}
