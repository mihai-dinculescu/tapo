use serde::Serialize;

use crate::error::Error;
use crate::requests::color::{Color, COLOR_MAP};
use crate::HandlerExt;

/// Builder that is used by the [`crate::ColorLightHandler::set`] API to set multiple properties in a single request.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ColorLightSetDeviceInfoParams {
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

impl ColorLightSetDeviceInfoParams {
    /// Turns *on* the device. [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    pub fn on(mut self) -> Self {
        self.device_on = Some(true);
        self
    }

    /// Turns *off* the device. [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    pub fn off(mut self) -> Self {
        self.device_on = Some(false);
        self
    }

    /// Sets the *brightness*. [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    /// The device will also be turned *on*, unless [`ColorLightSetDeviceInfoParams::off`] is called.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub fn brightness(mut self, value: u8) -> Self {
        self.brightness = Some(value);
        self
    }

    /// Sets the *color*. [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    /// The device will also be turned *on*, unless [`ColorLightSetDeviceInfoParams::off`] is called.
    ///
    /// # Arguments
    ///
    /// * `color` - one of [crate::requests::Color]
    pub fn color(mut self, color: Color) -> Self {
        let (hue, saturation, color_temperature) = *COLOR_MAP
            .get(&color)
            .unwrap_or_else(|| panic!("Failed to find the color definition for {color:?}"));

        self.hue = hue;
        self.saturation = saturation;
        self.color_temperature = color_temperature;

        self
    }

    /// Sets the *hue* and *saturation*. [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    /// The device will also be turned *on*, unless [`ColorLightSetDeviceInfoParams::off`] is called.
    ///
    /// # Arguments
    ///
    /// * `hue` - between 1 and 360
    /// * `saturation` - between 1 and 100
    pub fn hue_saturation(mut self, hue: u16, saturation: u8) -> Self {
        self.hue = Some(hue);
        self.saturation = Some(saturation);
        self.color_temperature = Some(0);

        self
    }

    /// Sets the *color temperature*. [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    /// The device will also be turned *on*, unless [`ColorLightSetDeviceInfoParams::off`] is called.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - between 2500 and 6500
    pub fn color_temperature(mut self, value: u16) -> Self {
        self.hue = Some(0);
        self.saturation = Some(100);
        self.color_temperature = Some(value);

        self
    }

    /// Performs a request to apply the changes to the device.
    pub async fn send(self, client: &impl HandlerExt) -> Result<(), Error> {
        self.validate()?;
        let json = serde_json::to_value(&self)?;
        client.get_client().set_device_info(json).await
    }
}

impl ColorLightSetDeviceInfoParams {
    /// Creates a new [`ColorLightSetDeviceInfoParams`] builder.
    pub fn new() -> Self {
        Self::default()
    }

    fn validate(&self) -> Result<(), Error> {
        if self.device_on.is_none()
            && self.brightness.is_none()
            && self.hue.is_none()
            && self.saturation.is_none()
            && self.color_temperature.is_none()
        {
            return Err(Error::Validation {
                field: "DeviceInfoParams".to_string(),
                message: "requires at least one property".to_string(),
            });
        }

        if let Some(brightness) = self.brightness {
            if !(1..=100).contains(&brightness) {
                return Err(Error::Validation {
                    field: "brightness".to_string(),
                    message: "must be between 1 and 100".to_string(),
                });
            }
        }

        if let Some(hue) = self.hue {
            if self.color_temperature.unwrap_or_default() == 0 && !(1..=360).contains(&hue) {
                return Err(Error::Validation {
                    field: "hue".to_string(),
                    message: "must be between 1 and 360".to_string(),
                });
            }
        }

        if let Some(saturation) = self.saturation {
            if !(1..=100).contains(&saturation) {
                return Err(Error::Validation {
                    field: "saturation".to_string(),
                    message: "must be between 1 and 100".to_string(),
                });
            }
        }

        if let Some(color_temperature) = self.color_temperature {
            if self.hue.unwrap_or_default() == 0
                && self.saturation.unwrap_or(100) == 100
                && !(2500..=6500).contains(&color_temperature)
            {
                return Err(Error::Validation {
                    field: "color_temperature".to_string(),
                    message: "must be between 2500 and 6500".to_string(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::ApiClientExt;

    use super::*;

    #[derive(Debug)]
    struct MockApiClient;

    #[async_trait]
    impl ApiClientExt for MockApiClient {
        async fn set_device_info(&self, _: serde_json::Value) -> Result<(), Error> {
            Ok(())
        }
    }

    #[derive(Debug)]
    struct MockHandler;

    impl HandlerExt for MockHandler {
        fn get_client(&self) -> &dyn ApiClientExt {
            &MockApiClient
        }
    }

    #[tokio::test]
    async fn hue_saturation_overrides_color_temperature() {
        let params = ColorLightSetDeviceInfoParams::new();

        let params = params.color_temperature(3000);
        let params = params.hue_saturation(50, 50);

        assert_eq!(params.hue, Some(50));
        assert_eq!(params.saturation, Some(50));
        assert_eq!(params.color_temperature, Some(0));

        assert!(params.send(&MockHandler).await.is_ok())
    }

    #[tokio::test]
    async fn color_temperature_overrides_hue_saturation() {
        let params = ColorLightSetDeviceInfoParams::new();

        let params = params.hue_saturation(50, 50);
        let params = params.color_temperature(3000);

        assert_eq!(params.hue, Some(0));
        assert_eq!(params.saturation, Some(100));
        assert_eq!(params.color_temperature, Some(3000));

        assert!(params.send(&MockHandler).await.is_ok())
    }

    #[tokio::test]
    async fn no_property_validation() {
        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "DeviceInfoParams" && message == "requires at least one property"
        ));
    }

    #[tokio::test]
    async fn brightness_validation() {
        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.brightness(0).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "brightness" && message == "must be between 1 and 100"
        ));

        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.brightness(101).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "brightness" && message == "must be between 1 and 100"
        ));
    }

    #[tokio::test]
    async fn hue_validation() {
        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.hue_saturation(0, 50).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "hue" && message == "must be between 1 and 360"
        ));

        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.hue_saturation(361, 50).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "hue" && message == "must be between 1 and 360"
        ));
    }

    #[tokio::test]
    async fn saturation_validation() {
        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.hue_saturation(1, 0).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "saturation" && message == "must be between 1 and 100"
        ));

        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.hue_saturation(1, 101).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "saturation" && message == "must be between 1 and 100"
        ));
    }

    #[tokio::test]
    async fn color_temperature_validation() {
        let params: ColorLightSetDeviceInfoParams = ColorLightSetDeviceInfoParams::new();
        let result = params.color_temperature(2499).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "color_temperature" && message == "must be between 2500 and 6500"
        ));

        let params = ColorLightSetDeviceInfoParams::new();
        let result = params.color_temperature(6501).send(&MockHandler).await;
        assert!(matches!(
            result.err(),
            Some(Error::Validation { field, message }) if field == "color_temperature" && message == "must be between 2500 and 6500"
        ));
    }
}
