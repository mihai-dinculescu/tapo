use std::fmt;

use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::de;
use serde::{Deserialize, Serialize};
use tapo::requests::Color;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DevicesList {
    /// Supported devices found on the network.
    pub devices: Vec<Device>,
    /// Devices that are currently unsupported.
    pub unsupported: Vec<UnsupportedDevice>,
    /// Errors encountered during discovery.
    pub errors: Vec<DiscoveryError>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum SetCapability {
    /// Set the device brightness (1-100).
    Brightness,
    /// Set the device color using a preset name.
    Color,
    /// Turn the device on or off.
    OnOff,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum GetCapability {
    /// Read the device's current state.
    DeviceInfo,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Device {
    /// Unique device identifier.
    pub id: String,
    /// User-assigned device name.
    pub name: String,
    /// Device model (e.g. "L530", "P110").
    pub model: String,
    /// Device IP address on the local network.
    pub ip: String,
    /// Capabilities that can be set on this device.
    pub set_capabilities: Vec<SetCapability>,
    /// Capabilities that can be read from this device.
    pub get_capabilities: Vec<GetCapability>,
    /// Child devices (e.g. individual plugs on a power strip).
    pub children: Vec<ChildDevice>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ChildDevice {
    /// Unique child device identifier.
    pub id: String,
    /// User-assigned child device name.
    pub name: String,
    /// Child device model.
    pub model: String,
    /// Capabilities that can be set on this child device.
    pub set_capabilities: Vec<SetCapability>,
    /// Capabilities that can be read from this child device.
    pub get_capabilities: Vec<GetCapability>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UnsupportedDevice {
    /// Device IP address on the local network.
    pub ip: String,
    /// Device model string.
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DiscoveryError {
    /// IP address of the device that encountered the error.
    pub ip: String,
    /// Error description.
    pub message: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckDeviceParams {
    /// Device ID from `list_devices`.
    pub id: String,
    /// Device IP address from `list_devices`.
    pub ip: String,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum SetCapabilityRequest {
    /// Set the device brightness. Also turns the device on if it's off.
    Brightness {
        /// Brightness level.
        #[schemars(range(min = 1, max = 100))]
        value: u8,
    },
    /// Set the device color using a preset name. Also turns the device on if it's off.
    Color {
        /// Preset color name.
        value: Color,
    },
    /// Turn the device on or off.
    OnOff {
        /// `true` to turn on, `false` to turn off.
        value: bool,
    },
}

impl fmt::Display for SetCapabilityRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Brightness { value } => write!(f, "Brightness({value})"),
            Self::Color { value } => write!(f, "Color({value:?})"),
            Self::OnOff { value } => {
                write!(f, "{}", if *value { "On" } else { "Off" })
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum GetCapabilityRequest {
    /// Read the device's current state (on/off, brightness, etc.).
    DeviceInfo,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControlDeviceParams {
    /// Device ID from `list_devices`.
    pub id: String,
    /// Device IP address from `list_devices`.
    pub ip: String,
    /// The set capabilities to apply.
    #[serde(deserialize_with = "deserialize_from_stringified_json")]
    #[schemars(length(min = 1))]
    pub capabilities: Vec<SetCapabilityRequest>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDeviceStateParams {
    /// Device ID from `list_devices`.
    pub id: String,
    /// Device IP address from `list_devices`.
    pub ip: String,
    /// The get capability to read.
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
