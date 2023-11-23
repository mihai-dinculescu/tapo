use serde::Serialize;

use crate::error::Error;

use crate::responses::TemperatureUnit;

#[derive(Debug, Default, Serialize)]
pub(crate) struct TrvSetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none", rename = "target_temp")]
    target_temperature: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frost_protection_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "temp_offset")]
    temperature_offset: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "min_temp")]
    min_temperature: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "min_control_temp")]
    min_control_temperature: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "max_control_temp")]
    max_control_temperature: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "temp_unit")]
    temperature_unit: Option<TemperatureUnit>,
}

impl TrvSetDeviceInfoParams {
    pub fn target_temperature(mut self, value: u8, unit: TemperatureUnit) -> Result<Self, Error> {
        self.target_temperature = Some(value);
        self.temperature_unit = Some(unit);
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
    pub fn temperature_offset(mut self, value: i8, unit: TemperatureUnit) -> Result<Self, Error> {
        self.temperature_offset = Some(value);
        self.temperature_unit = Some(unit);
        self.validate()
    }
    pub fn min_control_temperature(mut self, value: u8, unit: TemperatureUnit) -> Result<Self, Error> {
        self.min_control_temperature = Some(value);
        self.temperature_unit = Some(unit);
        self.validate()
    }
    pub fn max_control_temperature(mut self, value: u8, unit: TemperatureUnit) -> Result<Self, Error> {
        self.max_control_temperature = Some(value);
        self.temperature_unit = Some(unit);
        self.validate()
    }
}

impl TrvSetDeviceInfoParams {
    pub(crate) fn new() -> Self {
        Self {
            target_temperature: None,
            frost_protection_on: None,
            child_protection: None,
            temperature_offset: None,
            min_temperature: None,
            min_control_temperature: None,
            max_control_temperature: None,
            temperature_unit: None,
        }
    }
    
    pub fn validate(self) -> Result<Self, Error> {
        if let Some(temp_offset) = self.temperature_offset {
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
