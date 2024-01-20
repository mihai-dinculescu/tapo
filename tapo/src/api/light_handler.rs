use crate::api::ApiClient;
use crate::error::Error;
use crate::requests::LightSetDeviceInfoParams;
use crate::responses::{DeviceInfoLightResult, DeviceUsageEnergyMonitoringResult};

/// Handler for the [L510](https://www.tapo.com/en/search/?q=L510), [L520](https://www.tapo.com/en/search/?q=L520)
/// and [L610](https://www.tapo.com/en/search/?q=L610) devices.
pub struct LightHandler {
    client: ApiClient,
}

impl LightHandler {
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
        LightSetDeviceInfoParams::new(&self.client)
            .on()
            .send()
            .await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        LightSetDeviceInfoParams::new(&self.client)
            .off()
            .send()
            .await
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

    /// Returns *device info* as [`DeviceInfoLightResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`LightHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoLightResult, Error> {
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

    /// Sets the *brightness* and turns *on* the device.
    ///
    /// # Arguments
    ///
    /// * `brightness` - between 1 and 100
    pub async fn set_brightness(&self, brightness: u8) -> Result<(), Error> {
        LightSetDeviceInfoParams::new(&self.client)
            .brightness(brightness)
            .send()
            .await
    }
}
