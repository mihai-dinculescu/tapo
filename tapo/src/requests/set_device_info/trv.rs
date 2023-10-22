use serde::Serialize;

use crate::error::Error;

#[derive(Debug, Default, Serialize)]
pub(crate) struct TrvSetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_temp: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frost_protection_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_temp: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_control_temp: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_control_temp: Option<u8>,
}

impl TrvSetDeviceInfoParams {
    pub fn target_temp(mut self, value: u8) -> Result<Self, Error> {
        self.target_temp = Some(value);
        self.validate()
    }
    pub fn frost_protection_on(mut self, value: bool) -> Result<Self, Error> {
        self.frost_protection_on = Some(value);
        self.validate()
    }
    pub fn child_protection(mut self, value: bool) -> Result<Self, Error> {
        self.child_protection = Some(value);
        self.validate()
    }
    pub fn temp_offset(mut self, value: i8) -> Result<Self, Error> {
        self.temp_offset = Some(value);
        self.validate()
    }
    pub fn min_temp(mut self, value: u8) -> Result<Self, Error> {
        self.min_temp = Some(value);
        self.validate()
    }
    pub fn min_control_temp(mut self, value: u8) -> Result<Self, Error> {
        self.min_control_temp = Some(value);
        self.validate()
    }
    pub fn max_control_temp(mut self, value: u8) -> Result<Self, Error> {
        self.max_control_temp = Some(value);
        self.validate()
    }
}

impl TrvSetDeviceInfoParams {
    pub(crate) fn new() -> Self {
        Self {
            target_temp: None,
            frost_protection_on: None,
            child_protection: None,
            temp_offset: None,
            min_temp: None,
            min_control_temp: None,
            max_control_temp: None,
        }
    }
    
    pub fn validate(self) -> Result<Self, Error> {
        if let Some(temp_offset) = self.temp_offset {
            if temp_offset < -10 || temp_offset> 10 {
                return Err(Error::Validation {
                    field: "temp_offset".to_string(),
                    message: "must be between -10 and 10".to_string(),
                    
                });
            }
        }    
        Ok(self)
    }
}
