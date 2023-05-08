use std::marker::PhantomData;

use crate::api::{ApiClient, ApiClientExt, Authenticated, Unauthenticated};
use crate::error::Error;
use crate::requests::{EnergyDataInterval, GenericSetDeviceInfoParams};
use crate::responses::{
    DeviceUsageResult, EnergyDataResult, EnergyUsageResult, PlugDeviceInfoResult,
};

/// Handler for the [P110](https://www.tapo.com/en/search/?q=P110) & [P115](https://www.tapo.com/en/search/?q=P115) devices.
pub struct EnergyMonitoringPlugHandler<S = Unauthenticated> {
    client: ApiClient,
    status: PhantomData<S>,
}

impl<S> EnergyMonitoringPlugHandler<S> {
    pub(crate) fn new(client: ApiClient) -> Self {
        Self {
            client,
            status: PhantomData,
        }
    }

    /// Attempts to login. Each subsequent call will refresh the session.
    pub async fn login(mut self) -> Result<EnergyMonitoringPlugHandler<Authenticated>, Error> {
        self.client.login().await?;

        Ok(EnergyMonitoringPlugHandler {
            client: self.client,
            status: PhantomData,
        })
    }
}

impl EnergyMonitoringPlugHandler<Authenticated> {
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

    /// Gets *device info* as [`crate::responses::PlugDeviceInfoResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present, try [`crate::EnergyMonitoringPlugHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<PlugDeviceInfoResult, Error> {
        self.client.get_device_info::<PlugDeviceInfoResult>().await
    }

    /// Gets *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info_json().await
    }

    /// Gets *device usage* as [`crate::responses::DeviceUsageResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        self.client.get_device_usage().await
    }

    /// Gets *energy usage* as [`crate::responses::EnergyUsageResult`].
    pub async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        self.client.get_energy_usage().await
    }

    /// Gets *energy data* as [`crate::responses::EnergyDataResult`].
    pub async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> Result<EnergyDataResult, Error> {
        self.client.get_energy_data(interval).await
    }
}
