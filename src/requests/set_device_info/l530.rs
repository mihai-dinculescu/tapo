use anyhow::Context;
use serde::Serialize;

use crate::requests::color::{Color, COLOR_MAP};

#[derive(Debug, Default, Serialize)]
pub(crate) struct L530SetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saturation: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "color_temp")]
    color_temperature: Option<u16>,
}

impl L530SetDeviceInfoParams {
    pub fn brightness(value: u8) -> anyhow::Result<Self> {
        Self {
            brightness: Some(value),
            ..Default::default()
        }
        .validate()
    }

    pub fn color(color: Color) -> anyhow::Result<Self> {
        let (hue, saturation, color_temperature) = *COLOR_MAP
            .get(&color)
            .context("failed to find the color properties")?;

        Self {
            hue,
            saturation,
            color_temperature,
            ..Default::default()
        }
        .validate()
    }

    pub fn hue_saturation(hue: u16, saturation: u8) -> anyhow::Result<Self> {
        Self {
            hue: Some(hue),
            saturation: Some(saturation),
            color_temperature: None,
            ..Default::default()
        }
        .validate()
    }

    pub fn color_temperature(value: u16) -> anyhow::Result<Self> {
        Self {
            hue: None,
            saturation: None,
            color_temperature: Some(value),
            ..Default::default()
        }
        .validate()
    }

    pub fn validate(self) -> anyhow::Result<Self> {
        if self.brightness.is_none()
            && self.hue.is_none()
            && self.saturation.is_none()
            && self.color_temperature.is_none()
        {
            return Err(anyhow::anyhow!(
                "DeviceInfoParams requires at least one property"
            ));
        }

        if self.color_temperature.is_some() && (self.hue.is_some() || self.saturation.is_some()) {
            return Err(anyhow::anyhow!(
                "'color_temperature' cannot be set together with 'hue' or 'saturation'"
            ));
        }

        if (self.hue.is_some() && self.saturation.is_none())
            || (self.hue.is_none() && self.saturation.is_some())
        {
            return Err(anyhow::anyhow!(
                "'hue' and 'saturation' must be both set or unset"
            ));
        }

        if let Some(brightness) = self.brightness {
            if !(1..=100).contains(&brightness) {
                return Err(anyhow::anyhow!("'brightness' must be between 1 and 100"));
            }
        }

        if let Some(hue) = self.hue {
            if !(1..=360).contains(&hue) {
                return Err(anyhow::anyhow!("'hue' must be between 1 and 360"));
            }
        }

        if let Some(saturation) = self.saturation {
            if !(1..=100).contains(&saturation) {
                return Err(anyhow::anyhow!("'saturation' must be between 1 and 100"));
            }
        }

        if let Some(color_temp) = self.color_temperature {
            if !(2500..=6500).contains(&color_temp) {
                return Err(anyhow::anyhow!(
                    "'color_temperature' must be between 2500 and 6500"
                ));
            }
        }

        Ok(self)
    }
}
