use anyhow::Context;
use serde::Serialize;

use crate::api::ApiClient;
use crate::devices::L530;
use crate::requests::color::{Color, COLOR_MAP};

/// Builder that is used by the [`crate::ApiClient<L530>::set`] API to set multiple properties in a single request.
#[derive(Debug, Serialize)]
pub struct L530SetDeviceInfoParams<'a> {
    #[serde(skip)]
    client: &'a ApiClient<L530>,
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

impl<'a> L530SetDeviceInfoParams<'a> {
    /// Turns *on* the device. `send` must be called at the end to apply the changes.
    pub fn on(mut self) -> Self {
        self.device_on = Some(true);
        self
    }

    /// Turns *off* the device. `send` must be called at the end to apply the changes.
    pub fn off(mut self) -> Self {
        self.device_on = Some(false);
        self
    }

    /// Sets the *brightness*. `send` must be called at the end to apply the changes.
    ///
    /// # Arguments
    ///
    /// * `brightness` - *u8*; between 1 and 100
    pub fn brightness(mut self, value: u8) -> anyhow::Result<Self> {
        self.brightness = Some(value);
        self.validate()
    }

    /// Sets the *color*. `send` must be called at the end to apply the changes.
    ///
    /// # Arguments
    ///
    /// * `color` - [crate::requests::Color]
    pub fn color(mut self, color: Color) -> anyhow::Result<Self> {
        let (hue, saturation, color_temperature) = *COLOR_MAP
            .get(&color)
            .context("failed to find the color properties")?;

        self.hue = hue;
        self.saturation = saturation;
        self.color_temperature = color_temperature;

        self.validate()
    }

    /// Sets the *hue* and *saturation*. `send` must be called at the end to apply the changes.
    ///
    /// # Arguments
    ///
    /// * `hue` - *u16* between 1 and 360
    /// * `saturation` - *u8*; between 1 and 100
    pub fn hue_saturation(mut self, hue: u16, saturation: u8) -> anyhow::Result<Self> {
        self.hue = Some(hue);
        self.saturation = Some(saturation);
        self.color_temperature = None;

        self.validate()
    }

    /// Sets the *color temperature*. `send` must be called at the end to apply the changes.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - *u16*; between 2500 and 6500
    pub fn color_temperature(mut self, value: u16) -> anyhow::Result<Self> {
        self.hue = None;
        self.saturation = None;
        self.color_temperature = Some(value);

        self.validate()
    }

    /// Performs a request to apply the changes to the device.
    pub async fn send(self) -> anyhow::Result<()> {
        let json = serde_json::to_value(&self)?;
        self.client.set_device_info_internal(json).await
    }
}

impl<'a> L530SetDeviceInfoParams<'a> {
    pub(crate) fn new(client: &'a ApiClient<L530>) -> Self {
        Self {
            client,
            device_on: None,
            brightness: None,
            hue: None,
            saturation: None,
            color_temperature: None,
        }
    }

    fn validate(self) -> anyhow::Result<Self> {
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
