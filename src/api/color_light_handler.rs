use crate::api::{ApiClient, ApiClientExt};
use crate::error::Error;
use crate::requests::{Color, ColorLightSetDeviceInfoParams, GenericSetDeviceInfoParams};
use crate::responses::{DeviceUsageResult, L530DeviceInfoResult};

/// Handler for the [L530](https://www.tapo.com/en/search/?q=L530), [L630](https://www.tapo.com/en/search/?q=L630) and [L900](https://www.tapo.com/en/search/?q=L900) devices.
pub struct ColorLightHandler {
    client: ApiClient,
}

impl ColorLightHandler {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Attempts to refresh the authentication session.
    pub async fn login(mut self) -> Result<Self, Error> {
        let session = self.client.get_session_ref()?;
        self.client.login(session.url.clone()).await?;

        Ok(self)
    }

    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        self.client.set_device_info(json).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        self.client.set_device_info(json).await
    }

    /// Gets *device info* as [`crate::responses::L530DeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::ColorLightHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<L530DeviceInfoResult, Error> {
        self.client.get_device_info().await
    }

    /// Gets *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Gets *device usage* as [`crate::responses::DeviceUsageResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        self.client.get_device_usage().await
    }

    /// Returns a [`crate::requests::ColorLightSetDeviceInfoParams`] builder that allows multiple properties to be set in a single request.
    /// `send` must be called at the end to apply the changes.
    ///
    /// # Example
    /// ```rust,no_run
    /// use tapo::ApiClient;
    /// use tapo::requests::Color;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let device = ApiClient::new(
    ///         "192.168.1.100",
    ///         "tapo-username@example.com",
    ///         "tapo-password",
    ///     )?
    ///     .l530()
    ///     .login()
    ///     .await?;
    ///
    ///     device
    ///     .set()
    ///     .on()
    ///     .brightness(50)
    ///     .color(Color::HotPink)
    ///     .send()
    ///     .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set(&self) -> ColorLightSetDeviceInfoParams {
        ColorLightSetDeviceInfoParams::new(&self.client)
    }

    /// Sets the *brightness* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new(&self.client)
            .brightness(brightness)
            .send()
            .await
    }

    /// Sets the *color* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `color` - [crate::requests::Color]
    pub async fn set_color(&self, color: Color) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new(&self.client)
            .color(color)
            .send()
            .await
    }

    /// Sets the *hue*, *saturation* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `hue` - between 1 and 360
    /// * `saturation` - between 1 and 100
    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new(&self.client)
            .hue_saturation(hue, saturation)
            .send()
            .await
    }

    /// Sets the *color temperature* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `color_temperature` - between 2500 and 6500
    pub async fn set_color_temperature(&self, color_temperature: u16) -> Result<(), Error> {
        ColorLightSetDeviceInfoParams::new(&self.client)
            .color_temperature(color_temperature)
            .send()
            .await
    }
}
