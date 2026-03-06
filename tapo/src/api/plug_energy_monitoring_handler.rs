use crate::error::Error;
use crate::requests::{EnergyDataInterval, GenericSetDeviceInfoParams, PowerDataInterval};
use crate::responses::{
    CurrentPowerResult, DeviceInfoPlugEnergyMonitoringResult, DeviceUsageEnergyMonitoringResult,
    EnergyDataResult, EnergyUsageResult, PowerDataResult,
};

use super::ApiClientExt;

tapo_handler! {
    /// Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
    /// [P110M](https://www.tapo.com/en/search/?q=P110M) and
    /// [P115](https://www.tapo.com/en/search/?q=P115) devices.
    PlugEnergyMonitoringHandler(DeviceInfoPlugEnergyMonitoringResult),
    device_usage = DeviceUsageEnergyMonitoringResult,
    device_management,
}

impl PlugEnergyMonitoringHandler {
    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        self.client.read().await.set_device_info(json).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        self.client.read().await.set_device_info(json).await
    }

    /// Returns *current power* as [`CurrentPowerResult`].
    pub async fn get_current_power(&self) -> Result<CurrentPowerResult, Error> {
        self.client.read().await.get_current_power().await
    }

    /// Returns *energy usage* as [`EnergyUsageResult`].
    pub async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        self.client.read().await.get_energy_usage().await
    }

    /// Returns *energy data* as [`EnergyDataResult`].
    pub async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> Result<EnergyDataResult, Error> {
        self.client.read().await.get_energy_data(interval).await
    }

    /// Returns *power data* as [`PowerDataResult`].
    pub async fn get_power_data(
        &self,
        interval: PowerDataInterval,
    ) -> Result<PowerDataResult, Error> {
        self.client.read().await.get_power_data(interval).await
    }
}
