use crate::api::{ApiClient, ApiClientExt};
use crate::error::Error;
use crate::requests::{EnergyDataInterval, GenericSetDeviceInfoParams};
use crate::responses::{
    DeviceUsageResult, EnergyDataResult, EnergyUsageResult, PlugDeviceInfoResult,
};

/// Handler for the [P110](https://www.tapo.com/en/search/?q=P110) & [P115](https://www.tapo.com/en/search/?q=P115) devices.
pub struct EnergyMonitoringPlugHandler {
    client: ApiClient,
}

impl EnergyMonitoringPlugHandler {
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

    /// Returns *device info* as [`PlugDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`EnergyMonitoringPlugHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<PlugDeviceInfoResult, Error> {
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
}
