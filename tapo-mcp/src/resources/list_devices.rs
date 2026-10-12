use rmcp::model::{ReadResourceResult, Resource, ResourceContents};

use crate::discovery::DeviceCache;
use crate::errors::TapoMcpError;

pub const DEVICES_RESOURCE_URI: &str = "tapo://devices";
const JSON_MIME_TYPE: &str = "application/json";

pub fn build_devices_resource() -> Resource {
    Resource::new(DEVICES_RESOURCE_URI, "devices")
        .with_title("Tapo devices")
        .with_description("List available Tapo devices. The list is refreshed periodically in the background. If a device the user expects is missing, it may have joined after `discovered_at` and will appear after the next refresh.")
        .with_mime_type(JSON_MIME_TYPE)
}

pub fn read_devices(cache: &DeviceCache) -> Result<ReadResourceResult, TapoMcpError> {
    let devices = cache.get()?;
    let text = serde_json::to_string_pretty(&*devices)?;
    let contents =
        vec![ResourceContents::text(text, DEVICES_RESOURCE_URI).with_mime_type(JSON_MIME_TYPE)];
    Ok(ReadResourceResult::new(contents))
}
