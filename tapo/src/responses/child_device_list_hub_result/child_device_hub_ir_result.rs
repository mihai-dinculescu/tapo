use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{
    ChildDeviceHubResult, DecodableResultExt, IrRemoteResult, KE100Result, OtherResult, S200Result,
    S210Result, T31XResult, T100Result, T110Result, T300Result,
};

/// Child device result of a hub with an IR blaster (H110).
///
/// It has the same variants as [`ChildDeviceHubResult`], plus [`ChildDeviceHubIrResult::IrRemote`].
#[derive(Debug, Clone)]
pub enum ChildDeviceHubIrResult {
    /// IR remote paired with an H110 hub.
    IrRemote(Box<IrRemoteResult>),
    /// KE100 thermostatic radiator valve (TRV).
    KE100(Box<KE100Result>),
    /// S200B/S200D button switch.
    S200(Box<S200Result>),
    /// S210 light switch.
    S210(Box<S210Result>),
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

impl ChildDeviceHubIrResult {
    /// Returns the device ID.
    pub fn device_id(&self) -> &str {
        match self {
            ChildDeviceHubIrResult::IrRemote(d) => &d.device_id,
            ChildDeviceHubIrResult::KE100(d) => &d.device_id,
            ChildDeviceHubIrResult::S200(d) => &d.device_id,
            ChildDeviceHubIrResult::S210(d) => &d.device_id,
            ChildDeviceHubIrResult::T100(d) => &d.device_id,
            ChildDeviceHubIrResult::T110(d) => &d.device_id,
            ChildDeviceHubIrResult::T300(d) => &d.device_id,
            ChildDeviceHubIrResult::T31X(d) => &d.device_id,
            ChildDeviceHubIrResult::Other(d) => &d.device_id,
        }
    }

    /// Returns the device nickname.
    pub fn nickname(&self) -> &str {
        match self {
            ChildDeviceHubIrResult::IrRemote(d) => &d.nickname,
            ChildDeviceHubIrResult::KE100(d) => &d.nickname,
            ChildDeviceHubIrResult::S200(d) => &d.nickname,
            ChildDeviceHubIrResult::S210(d) => &d.nickname,
            ChildDeviceHubIrResult::T100(d) => &d.nickname,
            ChildDeviceHubIrResult::T110(d) => &d.nickname,
            ChildDeviceHubIrResult::T300(d) => &d.nickname,
            ChildDeviceHubIrResult::T31X(d) => &d.nickname,
            ChildDeviceHubIrResult::Other(d) => &d.nickname,
        }
    }

    /// Returns the model string (e.g. "S200B", "T310").
    /// For IR remotes, this is the kind of appliance the remote controls (e.g. "TV").
    pub fn model(&self) -> &str {
        match self {
            ChildDeviceHubIrResult::IrRemote(d) => &d.model,
            ChildDeviceHubIrResult::KE100(d) => &d.model,
            ChildDeviceHubIrResult::S200(d) => &d.model,
            ChildDeviceHubIrResult::S210(d) => &d.model,
            ChildDeviceHubIrResult::T100(d) => &d.model,
            ChildDeviceHubIrResult::T110(d) => &d.model,
            ChildDeviceHubIrResult::T300(d) => &d.model,
            ChildDeviceHubIrResult::T31X(d) => &d.model,
            ChildDeviceHubIrResult::Other(d) => &d.model,
        }
    }
}

impl From<ChildDeviceHubResult> for ChildDeviceHubIrResult {
    fn from(value: ChildDeviceHubResult) -> Self {
        match value {
            ChildDeviceHubResult::KE100(d) => ChildDeviceHubIrResult::KE100(d),
            ChildDeviceHubResult::S200(d) => ChildDeviceHubIrResult::S200(d),
            ChildDeviceHubResult::S210(d) => ChildDeviceHubIrResult::S210(d),
            ChildDeviceHubResult::T100(d) => ChildDeviceHubIrResult::T100(d),
            ChildDeviceHubResult::T110(d) => ChildDeviceHubIrResult::T110(d),
            ChildDeviceHubResult::T300(d) => ChildDeviceHubIrResult::T300(d),
            ChildDeviceHubResult::T31X(d) => ChildDeviceHubIrResult::T31X(d),
            ChildDeviceHubResult::Other(d) => ChildDeviceHubIrResult::Other(d),
        }
    }
}

impl Serialize for ChildDeviceHubIrResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ChildDeviceHubIrResult::IrRemote(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::KE100(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::S200(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::S210(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::T100(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::T110(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::T300(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::T31X(d) => d.serialize(serializer),
            ChildDeviceHubIrResult::Other(d) => d.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ChildDeviceHubIrResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        // IR remotes are matched on `type` because their `model` is the kind of
        // appliance the remote controls (e.g. "TV"), not a Tapo model.
        if value.get("type").and_then(|t| t.as_str()) == Some("SMART.TAPOREMOTE") {
            return serde_json::from_value(value)
                .map(|r| ChildDeviceHubIrResult::IrRemote(Box::new(r)))
                .map_err(serde::de::Error::custom);
        }

        serde_json::from_value::<ChildDeviceHubResult>(value)
            .map(ChildDeviceHubIrResult::from)
            .map_err(serde::de::Error::custom)
    }
}

impl DecodableResultExt for ChildDeviceHubIrResult {
    fn decode(self) -> Result<Self, Error> {
        match self {
            ChildDeviceHubIrResult::IrRemote(device) => {
                Ok(ChildDeviceHubIrResult::IrRemote(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::KE100(device) => {
                Ok(ChildDeviceHubIrResult::KE100(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::S200(device) => {
                Ok(ChildDeviceHubIrResult::S200(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::S210(device) => {
                Ok(ChildDeviceHubIrResult::S210(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::T100(device) => {
                Ok(ChildDeviceHubIrResult::T100(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::T110(device) => {
                Ok(ChildDeviceHubIrResult::T110(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::T300(device) => {
                Ok(ChildDeviceHubIrResult::T300(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::T31X(device) => {
                Ok(ChildDeviceHubIrResult::T31X(Box::new(device.decode()?)))
            }
            ChildDeviceHubIrResult::Other(device) => {
                Ok(ChildDeviceHubIrResult::Other(Box::new(device.decode()?)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_ir_child_falls_back_to_child_device_hub_result() {
        let json = r#"{"device_id":"0000","model":"UNKNOWN","nickname":"VGVzdA=="}"#;

        let parsed: ChildDeviceHubIrResult = serde_json::from_str(json).unwrap();

        assert!(matches!(parsed, ChildDeviceHubIrResult::Other(_)));
    }
}
