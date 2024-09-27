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

/// Hub child device list result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChildDeviceListHubResult {
    /// Hub child devices
    #[serde(rename = "child_device_list")]
    pub devices: Vec<ChildDeviceHubResult>,
}

impl DecodableResultExt for ChildDeviceListHubResult {
    fn decode(self) -> Result<Self, Error> {
        Ok(ChildDeviceListHubResult {
            devices: self
                .devices
                .into_iter()
                .map(|d| d.decode())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TapoResponseExt for ChildDeviceListHubResult {}

/// Device status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[allow(missing_docs)]
pub enum Status {
    Online,
    Offline,
}

/// Hub child device result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "model")]
pub enum ChildDeviceHubResult {
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
    /// T310 temperature and humidity sensor.
    T310(Box<T31XResult>),
    /// T315 temperature and humidity sensor.
    T315(Box<T31XResult>),
    /// Catch-all for currently unsupported devices.
    /// Please open an issue if you need support for a new device.
    #[serde(other)]
    Other,
}

impl DecodableResultExt for ChildDeviceHubResult {
    fn decode(self) -> Result<Self, Error> {
        match self {
            ChildDeviceHubResult::KE100(device) => {
                Ok(ChildDeviceHubResult::KE100(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::S200B(device) => {
                Ok(ChildDeviceHubResult::S200B(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::T100(device) => {
                Ok(ChildDeviceHubResult::T100(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::T110(device) => {
                Ok(ChildDeviceHubResult::T110(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::T300(device) => {
                Ok(ChildDeviceHubResult::T300(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::T310(device) => {
                Ok(ChildDeviceHubResult::T310(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::T315(device) => {
                Ok(ChildDeviceHubResult::T315(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::Other => Ok(ChildDeviceHubResult::Other),
        }
    }
}
