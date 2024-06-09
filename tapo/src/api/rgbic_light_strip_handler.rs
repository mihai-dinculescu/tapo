use crate::error::Error;
use crate::requests::{Color, ColorLightSetDeviceInfoParams, LightingEffect};
use crate::responses::{DeviceInfoRgbicLightStripResult, DeviceUsageEnergyMonitoringResult};

use super::{ApiClient, ApiClientExt, HandlerExt};

/// Handler for the [L920](https://www.tapo.com/en/search/?q=L920) and [L930](https://www.tapo.com/en/search/?q=L930) devices.
pub struct RgbicLightStripHandler {
    client: ApiClient,
}

impl RgbicLightStripHandler {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Refreshes the authentication session.
    pub async fn refresh_session(&mut self) -> Result<&mut Self, Error> {
        self.client.refresh_session().await?;
        Ok(self)
    }

    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new().on().send(self).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new().off().send(self).await
    }

    /// *Hardware resets* the device.
    ///
    /// **Warning**: This action will reset the device to its factory settings.
    /// The connection to the Wi-Fi network and the Tapo app will be lost,
    /// and the device will need to be reconfigured.
    ///
    /// This feature is especially useful when the device is difficult to access
    /// and requires reconfiguration.
    pub async fn device_reset(&self) -> Result<(), Error> {
        self.client.device_reset().await
    }

    /// Returns *device info* as [`DeviceInfoRgbicLightStripResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`RgbicLightStripHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoRgbicLightStripResult, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device usage* as [`DeviceUsageEnergyMonitoringResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageEnergyMonitoringResult, Error> {
        self.client.get_device_usage().await
    }

    /// Returns a [`ColorLightSetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// [`ColorLightSetDeviceInfoParams::send`] must be called at the end to apply the changes.
    /// For *lighting effects*, use [`RgbicLightStripHandler::set_lighting_effect`] instead.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # use tapo::requests::Color;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    /// #     .l930("192.168.1.100")
    /// #     .await?;
    /// device
    ///     .set()
    ///     .brightness(50)
    ///     .color(Color::HotPink)
    ///     .send(&device)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set(&self) -> ColorLightSetDeviceInfoParams {
        ColorLightSetDeviceInfoParams::new()
    }

    /// Sets the *brightness* and turns *on* the device.
    /// Pre-existing *lighting effect* will be removed.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .brightness(brightness)
            .send(self)
            .await
    }

    /// Sets the *color* and turns *on* the device.
    /// Pre-existing *lighting effect* will be removed.
    ///
    /// # Arguments
    ///
    /// * `color` - one of [crate::requests::Color] as defined in the Google Home app
    pub async fn set_color(&self, color: Color) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .color(color)
            .send(self)
            .await
    }

    /// Sets the *hue*, *saturation* and turns *on* the device.
    /// Pre-existing *lighting effect* will be removed.
    ///
    /// # Arguments
    ///
    /// * `hue` - between 0 and 360
    /// * `saturation` - between 1 and 100
    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .hue_saturation(hue, saturation)
            .send(self)
            .await
    }

    /// Sets the *color temperature* and turns *on* the device.
    /// Pre-existing *lighting effect* will be removed.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - between 2500 and 6500
    pub async fn set_color_temperature(&self, color_temperature: u16) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new()
            .color_temperature(color_temperature)
            .send(self)
            .await
    }

    /// Sets a *lighting effect* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `lighting_effect` - [crate::requests::LightingEffectPreset] or [crate::requests::LightingEffect].
    pub async fn set_lighting_effect(
        &self,
        lighting_effect: impl Into<LightingEffect>,
    ) -> Result<(), Error> {
        self.client
            .set_lighting_effect(lighting_effect.into())
            .await
    }
}

impl HandlerExt for RgbicLightStripHandler {
    fn get_client(&self) -> &dyn ApiClientExt {
        &self.client
    }
}
