use rmcp::model::{CallToolResult, ContentBlock};

use crate::discovery::DeviceCache;
use crate::errors::TapoMcpError;

pub fn list_devices(cache: &DeviceCache) -> Result<CallToolResult, TapoMcpError> {
    let devices = cache.get()?;
    let content = vec![ContentBlock::text(serde_json::to_string(&*devices)?)];
    Ok(CallToolResult::success(content))
}
