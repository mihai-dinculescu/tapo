use serde::Serialize;

use crate::error::Error;
use crate::responses::encode_value;

/// The longest name that `set_nickname` sends, in UTF-8 bytes. One byte under
/// the P110M's limit, which takes 129 bytes and rejects 130.
const MAX_NICKNAME_BYTES: usize = 128;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GenericSetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_on: Option<bool>,
    /// The name of the device, base64-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

impl GenericSetDeviceInfoParams {
    pub fn device_on(value: bool) -> Result<Self, Error> {
        Self {
            device_on: Some(value),
            ..Default::default()
        }
        .validate()
    }

    pub fn nickname(value: &str) -> Result<Self, Error> {
        if value.len() > MAX_NICKNAME_BYTES {
            return Err(Error::Validation {
                field: "nickname".to_string(),
                message: format!("Must be at most {MAX_NICKNAME_BYTES} bytes in UTF-8"),
            });
        }

        Self {
            nickname: Some(encode_value(value)),
            ..Default::default()
        }
        .validate()
    }

    pub fn validate(self) -> Result<Self, Error> {
        if self.device_on.is_none() && self.nickname.is_none() {
            return Err(Error::Validation {
                field: "DeviceInfoParams".to_string(),
                message: "Requires at least one property".to_string(),
            });
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn the_nickname_is_sent_base64_encoded() {
        assert_eq!(
            serde_json::to_value(GenericSetDeviceInfoParams::nickname("Küche").unwrap()).unwrap(),
            json!({"nickname": "S8O8Y2hl"})
        );
    }

    #[test]
    fn an_empty_nickname_is_sent() {
        assert_eq!(
            serde_json::to_value(GenericSetDeviceInfoParams::nickname("").unwrap()).unwrap(),
            json!({"nickname": ""})
        );
    }

    #[test]
    fn a_nickname_is_limited_to_128_bytes() {
        assert!(GenericSetDeviceInfoParams::nickname(&"A".repeat(128)).is_ok());
        assert!(GenericSetDeviceInfoParams::nickname(&"ü".repeat(64)).is_ok());

        for nickname in ["A".repeat(129), format!("{}A", "ü".repeat(64))] {
            assert!(matches!(
                GenericSetDeviceInfoParams::nickname(&nickname),
                Err(Error::Validation { field, .. }) if field == "nickname"
            ));
        }
    }
}
