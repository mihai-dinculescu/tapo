mod ke100_result;
mod other_result;
mod s200_result;
mod t100_result;
mod t110_result;
mod t300_result;
mod t31x_result;

pub use ke100_result::*;
pub use other_result::*;
pub use s200_result::*;
pub use t31x_result::*;
pub use t100_result::*;
pub use t110_result::*;
pub use t300_result::*;

use serde::de::Deserializer;
use serde::ser::Serializer;
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
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, eq_int)
)]
#[allow(missing_docs)]
pub enum Status {
    Online,
    Offline,
}

/// Hub child device result.
#[derive(Debug, Clone)]
pub enum ChildDeviceHubResult {
    /// KE100 thermostatic radiator valve (TRV).
    KE100(Box<KE100Result>),
    /// S200B/S200D button switch.
    S200(Box<S200Result>),
    /// T100 motion sensor.
    T100(Box<T100Result>),
    /// T110 contact sensor.
    T110(Box<T110Result>),
    /// T300 water sensor.
    T300(Box<T300Result>),
    /// T310/T315 temperature and humidity sensor.
    T31X(Box<T31XResult>),
    /// Catch-all for unsupported devices. Open a GitHub issue to request support.
    Other(Box<OtherResult>),
}

impl Serialize for ChildDeviceHubResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ChildDeviceHubResult::KE100(d) => d.serialize(serializer),
            ChildDeviceHubResult::S200(d) => d.serialize(serializer),
            ChildDeviceHubResult::T100(d) => d.serialize(serializer),
            ChildDeviceHubResult::T110(d) => d.serialize(serializer),
            ChildDeviceHubResult::T300(d) => d.serialize(serializer),
            ChildDeviceHubResult::T31X(d) => d.serialize(serializer),
            ChildDeviceHubResult::Other(d) => d.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ChildDeviceHubResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let model = value.get("model").and_then(|m| m.as_str()).unwrap_or("");

        match model {
            "KE100" => serde_json::from_value(value)
                .map(|r| ChildDeviceHubResult::KE100(Box::new(r)))
                .map_err(serde::de::Error::custom),
            "S200B" | "S200D" => serde_json::from_value(value)
                .map(|r| ChildDeviceHubResult::S200(Box::new(r)))
                .map_err(serde::de::Error::custom),
            "T100" => serde_json::from_value(value)
                .map(|r| ChildDeviceHubResult::T100(Box::new(r)))
                .map_err(serde::de::Error::custom),
            "T110" => serde_json::from_value(value)
                .map(|r| ChildDeviceHubResult::T110(Box::new(r)))
                .map_err(serde::de::Error::custom),
            "T300" => serde_json::from_value(value)
                .map(|r| ChildDeviceHubResult::T300(Box::new(r)))
                .map_err(serde::de::Error::custom),
            "T310" | "T315" => serde_json::from_value(value)
                .map(|r| ChildDeviceHubResult::T31X(Box::new(r)))
                .map_err(serde::de::Error::custom),
            _ => serde_json::from_value(value)
                .map(|r| ChildDeviceHubResult::Other(Box::new(r)))
                .map_err(serde::de::Error::custom),
        }
    }
}

impl DecodableResultExt for ChildDeviceHubResult {
    fn decode(self) -> Result<Self, Error> {
        match self {
            ChildDeviceHubResult::KE100(device) => {
                Ok(ChildDeviceHubResult::KE100(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::S200(device) => {
                Ok(ChildDeviceHubResult::S200(Box::new(device.decode()?)))
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
            ChildDeviceHubResult::T31X(device) => {
                Ok(ChildDeviceHubResult::T31X(Box::new(device.decode()?)))
            }
            ChildDeviceHubResult::Other(device) => {
                Ok(ChildDeviceHubResult::Other(Box::new(device.decode()?)))
            }
        }
    }
}
