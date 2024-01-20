use crate::api::{ApiClient, ApiClientExt};
use crate::error::Error;
use crate::requests::{EnergyDataInterval, GenericSetDeviceInfoParams};
use crate::responses::{
    CurrentPowerResult, DeviceInfoPlugResult, DeviceUsageEnergyMonitoringResult, EnergyDataResult,
    EnergyUsageResult,
};

/// Handler for the [P110](https://www.tapo.com/en/search/?q=P110) & [P115](https://www.tapo.com/en/search/?q=P115) devices.
pub struct PlugEnergyMonitoringHandler {
    client: ApiClient,
}

impl PlugEnergyMonitoringHandler {
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
    /// If the deserialization fails, or if a property that you care about it's not present, try [`PlugEnergyMonitoringHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<DeviceInfoPlugResult, Error> {
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

    /// Returns *energy usage* as [`EnergyUsageResult`].
    pub async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        self.client.get_energy_usage().await
    }

    /// Returns *energy data* as [`EnergyDataResult`].
    pub async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> Result<EnergyDataResult, Error> {
        self.client.get_energy_data(interval).await
    }

    /// Returns *current power* as [`CurrentPowerResult`].
    pub async fn get_current_power(&self) -> Result<CurrentPowerResult, Error> {
        self.client.get_current_power().await
    }
}
