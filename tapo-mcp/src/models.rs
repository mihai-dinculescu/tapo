use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DevicesList {
    pub devices: Vec<Device>,
    pub unsupported: Vec<UnsupportedDevice>,
    pub errors: Vec<DiscoveryError>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum SetCapability {
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
pub enum SetCapabilityRequest {
    OnOff(bool),
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub enum GetCapabilityRequest {
    DeviceInfo,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControlDeviceParams {
    pub id: String,
    pub ip: String,
    pub capability: SetCapabilityRequest,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDeviceStateParams {
    pub id: String,
    pub ip: String,
    pub capability: GetCapabilityRequest,
}
