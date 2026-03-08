use rmcp::model::{CallToolResult, Content};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::CheckDeviceParams;
use crate::requests;

pub async fn check_device(
    config: &AppConfig,
    params: CheckDeviceParams,
) -> Result<CallToolResult, TapoMcpError> {
    requests::check_device(config, params).await?;
    let content = vec![Content::text("Check OK")];
    Ok(CallToolResult::success(content))
}
