use crate::error::Error;
use crate::requests::{EnergyDataInterval, PowerDataInterval};
use crate::responses::{
    CurrentPowerResult, DeviceInfoPlugEnergyMonitoringResult, DeviceUsageEnergyMonitoringResult,
    EnergyDataResult, EnergyUsageResult, PowerDataResult,
};

tapo_handler! {
    /// Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
    /// [P110M](https://www.tapo.com/en/search/?q=P110M) and
    /// [P115](https://www.tapo.com/en/search/?q=P115) devices.
    PlugEnergyMonitoringHandler(DeviceInfoPlugEnergyMonitoringResult),
    on_off,
    device_usage = DeviceUsageEnergyMonitoringResult,
    device_management,
}

impl PlugEnergyMonitoringHandler {
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
