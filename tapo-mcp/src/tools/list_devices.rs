use rmcp::model::{CallToolResult, Content};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::requests::get_devices;

pub async fn list_devices(config: &AppConfig) -> Result<CallToolResult, TapoMcpError> {
    let devices = get_devices(config).await?;
    let text = serde_json::to_string_pretty(&devices)?;
    let content = vec![Content::text(text)];
    Ok(CallToolResult::success(content))
}
