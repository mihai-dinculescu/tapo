use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::config::AppConfig;
use crate::models::CheckDeviceParams;
use crate::requests;

pub async fn check_device(
    config: &AppConfig,
    params: CheckDeviceParams,
) -> Result<CallToolResult, McpError> {
    requests::check_device(config, params).await?;
    Ok(CallToolResult::success(vec![]))
}
