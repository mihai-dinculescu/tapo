mod s200b_switch;
mod t100_sensor;
mod t110_sensor;
mod t31x_sensor;

pub use s200b_switch::*;
pub use t100_sensor::*;
pub use t110_sensor::*;
pub use t31x_sensor::*;

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, TapoResponseExt};

/// Child device list result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildDeviceListResult {
    /// Child devices
    #[serde(rename = "child_device_list")]
    pub devices: Vec<ChildDeviceResult>,
}

impl DecodableResultExt for ChildDeviceListResult {
    fn decode(self) -> Result<Self, Error> {
        Ok(ChildDeviceListResult {
            devices: self
                .devices
                .into_iter()
                .map(|d| d.decode())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TapoResponseExt for ChildDeviceListResult {}

/// Device status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub enum Status {
    Online,
    Offline,
}

/// Child device result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "model")]
pub enum ChildDeviceResult {
    /// S200B button switch.
    S200B(Box<S200BSwitch>),
    /// T100 motion sensor.
    T100(Box<T100Sensor>),
    /// T110 contact sensor.
    T110(Box<T110Sensor>),
    /// T310 temperature & humidity sensor.
    T310(Box<T31XSensor>),
    /// T315 temperature & humidity sensor.
    T315(Box<T31XSensor>),
    /// Catch-all for currently unsupported devices.
    /// Please open an issue if you need support for a new device.
    #[serde(other)]
    Other,
}

impl DecodableResultExt for ChildDeviceResult {
    fn decode(self) -> Result<Self, Error> {
        match self {
            ChildDeviceResult::S200B(device) => Ok(ChildDeviceResult::S200B(device.decode()?)),
            ChildDeviceResult::T100(device) => Ok(ChildDeviceResult::T100(device.decode()?)),
            ChildDeviceResult::T110(device) => Ok(ChildDeviceResult::T110(device.decode()?)),
            ChildDeviceResult::T310(device) => Ok(ChildDeviceResult::T310(device.decode()?)),
            ChildDeviceResult::T315(device) => Ok(ChildDeviceResult::T315(device.decode()?)),
            ChildDeviceResult::Other => Ok(ChildDeviceResult::Other),
        }
    }
}
