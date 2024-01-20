use crate::api::{ApiClient, ApiClientExt};
use crate::error::Error;
use crate::requests::GenericSetDeviceInfoParams;
use crate::responses::{DeviceInfoPlugResult, DeviceUsageResult};

/// Handler for the [P100](https://www.tapo.com/en/search/?q=P100) & [P105](https://www.tapo.com/en/search/?q=P105) devices.
pub struct PlugHandler {
    client: ApiClient,
}

impl PlugHandler {
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
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        self.client.set_device_info(json).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        self.client.set_device_info(json).await
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

    /// Returns *device info* as [`DeviceInfoPlugResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`PlugHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoPlugResult, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device usage* as [`DeviceUsageResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        self.client.get_device_usage().await
    }
}
