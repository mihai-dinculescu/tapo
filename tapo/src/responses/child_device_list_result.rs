mod ke100_result;
mod s200b_result;
mod t100_result;
mod t110_result;
mod t300_result;
mod t31x_result;

pub use ke100_result::*;
pub use s200b_result::*;
pub use t100_result::*;
pub use t110_result::*;
pub use t300_result::*;
pub use t31x_result::*;

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
    /// KE100 thermostatic radiator valve (TRV).
    KE100(Box<KE100Result>),
    /// S200B button switch.
    S200B(Box<S200BResult>),
    /// T100 motion sensor.
    T100(Box<T100Result>),
    /// T110 contact sensor.
    T110(Box<T110Result>),
    /// T300 water sensor.
    T300(Box<T300Result>),
    /// T310 temperature & humidity sensor.
    T310(Box<T31XResult>),
    /// T315 temperature & humidity sensor.
    T315(Box<T31XResult>),
    /// Catch-all for currently unsupported devices.
    /// Please open an issue if you need support for a new device.
    #[serde(other)]
    Other,
}

impl DecodableResultExt for ChildDeviceResult {
    fn decode(self) -> Result<Self, Error> {
        match self {
            ChildDeviceResult::KE100(device) => {
                Ok(ChildDeviceResult::KE100(Box::new(device.decode()?)))
            }
            ChildDeviceResult::S200B(device) => {
                Ok(ChildDeviceResult::S200B(Box::new(device.decode()?)))
            }
            ChildDeviceResult::T100(device) => {
                Ok(ChildDeviceResult::T100(Box::new(device.decode()?)))
            }
            ChildDeviceResult::T110(device) => {
                Ok(ChildDeviceResult::T110(Box::new(device.decode()?)))
            }
            ChildDeviceResult::T300(device) => {
                Ok(ChildDeviceResult::T300(Box::new(device.decode()?)))
            }
            ChildDeviceResult::T310(device) => {
                Ok(ChildDeviceResult::T310(Box::new(device.decode()?)))
            }
            ChildDeviceResult::T315(device) => {
                Ok(ChildDeviceResult::T315(Box::new(device.decode()?)))
            }
            ChildDeviceResult::Other => Ok(ChildDeviceResult::Other),
        }
    }
}
