use rmcp::model::CallToolResult;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
use crate::errors::TapoMcpError;
use crate::models::CheckDeviceParams;
use crate::requests;

pub async fn check_device(
    config: &AppConfig,
    locks: &DeviceLocks,
    params: CheckDeviceParams,
) -> Result<CallToolResult, TapoMcpError> {
    requests::with_device(config, locks, &params.id, &params.ip, async |_| Ok(())).await?;
    Ok(CallToolResult::success(vec![]))
}
