use serde::Serialize;

use crate::error::Error;

use crate::responses::TemperatureUnitKE100;

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
    temperature_unit: Option<TemperatureUnitKE100>,
}

impl TrvSetDeviceInfoParams {
    pub fn target_temperature(
        mut self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> Result<Self, Error> {
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
    pub fn temperature_offset(
        mut self,
        value: i8,
        unit: TemperatureUnitKE100,
    ) -> Result<Self, Error> {
        self.temperature_offset = Some(value);
        self.temperature_unit = Some(unit);
        self.validate()
    }
    pub fn min_control_temperature(
        mut self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> Result<Self, Error> {
        self.min_control_temperature = Some(value);
        self.temperature_unit = Some(unit);
        self.validate()
    }
    pub fn max_control_temperature(
        mut self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> Result<Self, Error> {
        self.max_control_temperature = Some(value);
        self.temperature_unit = Some(unit);
        self.validate()
    }
}

impl TrvSetDeviceInfoParams {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn validate(self) -> Result<Self, Error> {
        if let Some(temperature_offset) = self.temperature_offset {
            if !(-10..=10).contains(&temperature_offset) {
                return Err(Error::Validation {
                    field: "temperature_offset".to_string(),
                    message: "Must be between -10 and 10".to_string(),
                });
            }
        }
        Ok(self)
    }
}
