use rmcp::model::{ReadResourceResult, Resource, ResourceContents};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::requests::get_devices;

pub const DEVICES_RESOURCE_URI: &str = "tapo://devices";
const JSON_MIME_TYPE: &str = "application/json";

pub fn build_devices_resource() -> Resource {
    Resource::new(DEVICES_RESOURCE_URI, "devices")
        .with_title("Tapo devices")
        .with_description("List available Tapo devices.")
        .with_mime_type(JSON_MIME_TYPE)
}

pub async fn read_devices(config: &AppConfig) -> Result<ReadResourceResult, TapoMcpError> {
    let devices = get_devices(config).await?;
    let text = serde_json::to_string_pretty(&devices)?;
    let contents =
        vec![ResourceContents::text(text, DEVICES_RESOURCE_URI).with_mime_type(JSON_MIME_TYPE)];
    Ok(ReadResourceResult::new(contents))
}
