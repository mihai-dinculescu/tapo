use crate::error::{Error, TapoResponseError};
use crate::requests::{
    EmptyParams, EnergyDataInterval, GetEnergyDataParams, GetPowerDataParams, PowerDataInterval,
    TapoParams, TapoRequest,
};
use crate::responses::{
    CurrentPowerResult, DeviceUsageEnergyMonitoringResult, EnergyDataResult, EnergyDataResultRaw,
    EnergyUsageResult, PowerDataResult, PowerDataResultRaw, PowerStripPlugEnergyMonitoringResult,
};

tapo_child_handler! {
    /// Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
    /// [P316M](https://www.tp-link.com/us/search/?q=P316M) child plugs.
    PowerStripPlugEnergyMonitoringHandler(PowerStripPlugEnergyMonitoringResult),
    on_off,
}

impl PowerStripPlugEnergyMonitoringHandler {
    /// Returns *current power* as [`CurrentPowerResult`].
    pub async fn get_current_power(&self) -> Result<CurrentPowerResult, Error> {
        let request = TapoRequest::GetCurrentPower(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Returns *device usage* as [`DeviceUsageEnergyMonitoringResult`].
    pub async fn get_device_usage(&self) -> Result<DeviceUsageEnergyMonitoringResult, Error> {
        let request = TapoRequest::GetDeviceUsage(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Returns *energy usage* as [`EnergyUsageResult`].
    pub async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        let request = TapoRequest::GetEnergyUsage(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Returns *energy data* as [`EnergyDataResult`].
    pub async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> Result<EnergyDataResult, Error> {
        let params = GetEnergyDataParams::new(interval);
        let request = TapoRequest::GetEnergyData(TapoParams::new(params));

        self.client
            .read()
            .await
            .control_child::<EnergyDataResultRaw>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.try_into())?
    }

    /// Returns *power data* as [`PowerDataResult`].
    pub async fn get_power_data(
        &self,
        interval: PowerDataInterval,
    ) -> Result<PowerDataResult, Error> {
        let params = GetPowerDataParams::new(interval);
        let request = TapoRequest::GetPowerData(TapoParams::new(params));

        self.client
            .read()
            .await
            .control_child::<PowerDataResultRaw>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.try_into())?
    }
}
