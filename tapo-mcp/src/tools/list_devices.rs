use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, ContentBlock};

use crate::discovery::DeviceCache;

pub fn list_devices(cache: &DeviceCache) -> Result<CallToolResult, McpError> {
    let devices = cache.get()?;
    let content = vec![ContentBlock::json(&*devices)?];
    Ok(CallToolResult::success(content))
}
