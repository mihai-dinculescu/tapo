use std::sync::Arc;

use tokio::sync::RwLock;

use crate::api::ApiClient;
use crate::error::{Error, TapoResponseError};
use crate::requests::{
    EmptyParams, EnergyDataInterval, GenericSetDeviceInfoParams, GetEnergyDataParams,
    GetPowerDataParams, PowerDataInterval, TapoParams, TapoRequest,
};
use crate::responses::{
    CurrentPowerResult, DecodableResultExt, DeviceUsageEnergyMonitoringResult, EnergyDataResult,
    EnergyDataResultRaw, EnergyUsageResult, PowerDataResult, PowerDataResultRaw,
    PowerStripPlugEnergyMonitoringResult,
};

/// Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
/// [P316M](https://www.tp-link.com/us/search/?q=P316M) child plugs.
pub struct PowerStripPlugEnergyMonitoringHandler {
    client: Arc<RwLock<ApiClient>>,
    device_id: String,
}

impl PowerStripPlugEnergyMonitoringHandler {
    pub(crate) fn new(client: Arc<RwLock<ApiClient>>, device_id: String) -> Self {
        Self { client, device_id }
    }

    /// Returns *device info* as [`PowerStripPlugEnergyMonitoringResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present,
    /// try [`PowerStripPlugEnergyMonitoringHandler::get_device_info_json`].
    pub async fn get_device_info(&self) -> Result<PowerStripPlugEnergyMonitoringResult, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child::<PowerStripPlugEnergyMonitoringResult>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.decode())?
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

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
