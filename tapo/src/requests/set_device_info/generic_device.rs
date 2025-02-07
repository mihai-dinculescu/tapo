use serde::Serialize;

use crate::error::Error;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GenericSetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_on: Option<bool>,
}

impl GenericSetDeviceInfoParams {
    pub fn device_on(value: bool) -> Result<Self, Error> {
        Self {
            device_on: Some(value),
        }
        .validate()
    }

    pub fn validate(self) -> Result<Self, Error> {
        if self.device_on.is_none() {
            return Err(Error::Validation {
                field: "DeviceInfoParams".to_string(),
                message: "Requires at least one property".to_string(),
            });
        }

        Ok(self)
    }
}
