use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::de;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DevicesList {
    pub devices: Vec<Device>,
    pub unsupported: Vec<UnsupportedDevice>,
    pub errors: Vec<DiscoveryError>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum SetCapability {
    Brightness,
    OnOff,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum GetCapability {
    DeviceInfo,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub model: String,
    pub ip: String,
    pub set_capabilities: Vec<SetCapability>,
    pub get_capabilities: Vec<GetCapability>,
    pub children: Vec<ChildDevice>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ChildDevice {
    pub id: String,
    pub name: String,
    pub model: String,
    pub set_capabilities: Vec<SetCapability>,
    pub get_capabilities: Vec<GetCapability>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UnsupportedDevice {
    pub ip: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DiscoveryError {
    pub ip: String,
    pub message: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckDeviceParams {
    pub id: String,
    pub ip: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum SetCapabilityRequest {
    /// Set the device brightness (turns the device on).
    Brightness {
        #[schemars(range(min = 1, max = 100))]
        value: u8,
    },
    /// Turn the device on or off.
    OnOff { value: bool },
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub enum GetCapabilityRequest {
    DeviceInfo,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControlDeviceParams {
    pub id: String,
    pub ip: String,
    #[serde(deserialize_with = "deserialize_from_stringified_json")]
    pub capability: SetCapabilityRequest,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDeviceStateParams {
    pub id: String,
    pub ip: String,
    #[serde(deserialize_with = "deserialize_from_stringified_json")]
    pub capability: GetCapabilityRequest,
}

/// Deserializes a value that may have been stringified by the MCP client.
///
/// Some clients (e.g. Claude Code) use an XML-to-JSON pipeline that doesn't
/// coerce types based on the tool's `inputSchema`, causing nested objects to
/// arrive as JSON strings. This handles both proper JSON objects
/// (e.g. `{"type": "Brightness", "value": 100}`) and stringified JSON
/// (e.g. the string `'{"type": "Brightness", "value": 100}'`).
///
/// See: https://github.com/anthropics/claude-code/issues/32524
fn deserialize_from_stringified_json<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match &value {
        serde_json::Value::String(s) => serde_json::from_str(s).map_err(de::Error::custom),
        _ => serde_json::from_value(value).map_err(de::Error::custom),
    }
}
