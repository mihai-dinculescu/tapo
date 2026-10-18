use chrono_tz::Tz;

use crate::error::{Error, TapoResponseError};
use crate::requests::{
    EmptyParams, EnergyDataInterval, GetEnergyDataParams, GetPowerDataParams, PowerDataInterval,
    TapoParams, TapoRequest,
};
use crate::responses::{
    CurrentPowerResult, DeviceUsageEnergyMonitoringResult, EnergyDataResult, EnergyDataResultRaw,
    EnergyUsageResult, PowerDataResult, PowerDataResultRaw, PowerStripPlugEnergyMonitoringResult,
    TimezoneResult,
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

    /// Returns *energy data* as [`EnergyDataResult`].
    ///
    /// The dates in `interval` are read in the power strip's timezone, so the
    /// entries line up with the Tapo app wherever this code runs. Entries that
    /// start after the power strip's local time are left out.
    ///
    /// Pass the [`TimezoneResult::timezone`] field from
    /// [`PowerStripPlugEnergyMonitoringHandler::get_timezone`] to read it once
    /// for several calls. With `None`, each call reads it, which takes one
    /// extra request.
    ///
    /// # Errors
    ///
    /// Returns an error if the power strip reports a timezone name that is not
    /// in the IANA database.
    pub async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
        timezone: Option<Tz>,
    ) -> Result<EnergyDataResult, Error> {
        let timezone = match timezone {
            Some(timezone) => timezone,
            None => self.get_timezone().await?.timezone,
        };
        let params = GetEnergyDataParams::new(interval, &timezone)?;
        let request = TapoRequest::GetEnergyData(TapoParams::new(params));

        self.client
            .read()
            .await
            .control_child::<EnergyDataResultRaw>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .in_timezone(&timezone)
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

    /// Returns the power strip's *timezone* as [`TimezoneResult`]. The child
    /// plugs follow the power strip's clock, so this reads it from the power strip.
    ///
    /// # Errors
    ///
    /// Returns an error if the power strip reports a timezone name that is not
    /// in the IANA database.
    pub async fn get_timezone(&self) -> Result<TimezoneResult, Error> {
        self.client.read().await.get_timezone().await
    }
}
