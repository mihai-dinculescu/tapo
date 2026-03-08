use rmcp::model::{AnnotateAble, RawResource, ReadResourceResult, Resource, ResourceContents};

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::requests::get_devices;

pub const DEVICES_RESOURCE_URI: &str = "tapo://devices";
const JSON_MIME_TYPE: &str = "application/json";

pub fn build_devices_resource() -> Resource {
    let mut resource = RawResource::new(DEVICES_RESOURCE_URI, "devices");
    resource.title = Some("Tapo devices".to_string());
    resource.description = Some("List of configured Tapo devices.".to_string());
    resource.mime_type = Some(JSON_MIME_TYPE.to_string());
    resource.no_annotation()
}

pub async fn read_devices(config: &AppConfig) -> Result<ReadResourceResult, TapoMcpError> {
    let devices = get_devices(config).await?;
    let text = serde_json::to_string_pretty(&devices)?;
    let contents = vec![ResourceContents::TextResourceContents {
        uri: DEVICES_RESOURCE_URI.to_string(),
        mime_type: Some(JSON_MIME_TYPE.to_string()),
        text,
        meta: None,
    }];
    Ok(ReadResourceResult::new(contents))
}
