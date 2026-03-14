use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};

use crate::config::AppConfig;
use crate::requests::get_devices;

pub async fn list_devices(config: &AppConfig) -> Result<CallToolResult, McpError> {
    let devices = get_devices(config).await?;
    let content = vec![Content::json(devices)?];
    Ok(CallToolResult::success(content))
}
