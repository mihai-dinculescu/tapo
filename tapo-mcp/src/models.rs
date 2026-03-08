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
pub enum Capability {
    OnOff,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub model: String,
    pub ip: String,
    pub capabilities: Vec<Capability>,
    pub children: Vec<ChildDevice>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ChildDevice {
    pub id: String,
    pub name: String,
    pub model: String,
    pub capabilities: Vec<Capability>,
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
pub enum SetCapability {
    OnOff(bool),
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetDeviceStateParams {
    pub id: String,
    pub ip: String,
    pub capability: SetCapability,
}
