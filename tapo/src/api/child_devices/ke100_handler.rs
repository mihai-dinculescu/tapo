use crate::api::HubHandler;
use crate::error::{Error, TapoResponseError};
use crate::requests::{EmptyParams, TapoParams, TapoRequest, TrvSetDeviceInfoParams};
use crate::responses::{DecodableResultExt, KE100Result, TemperatureUnitKE100};

/// Handler for the [KE100](https://www.tp-link.com/en/search/?q=KE100) devices.
pub struct KE100Handler<'h> {
    hub_handler: &'h HubHandler,
    device_id: String,
}

impl<'h> KE100Handler<'h> {
    pub(crate) fn new(hub_handler: &'h HubHandler, device_id: String) -> Self {
        Self {
            hub_handler,
            device_id,
        }
    }

    /// Returns *device info* as [`KE100Result`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self) -> Result<KE100Result, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.hub_handler
            .control_child::<KE100Result>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.decode())?
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.hub_handler
            .control_child(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Sets the *target temperature*.
    ///
    /// # Arguments
    ///
    /// * `target_temperature` - between `min_control_temperature` and `max_control_temperature`
    /// * `temperature_unit`
    pub async fn set_target_temperature(
        &self,
        target_temperature: u8,
        temperature_unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let device_info = self.get_device_info().await?;

        if target_temperature < device_info.min_control_temperature
            || target_temperature > device_info.max_control_temperature
        {
            return Err(Error::Validation {
                field: "target_temperature".to_string(),
                message: format!("Target temperature must be between {} (min_control_temperature) and {} (max_control_temperature)", device_info.min_control_temperature, device_info.max_control_temperature),
            });
        }

        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new()
                .target_temperature(target_temperature, temperature_unit)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.hub_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets the *minimal control temperature*.
    ///
    /// # Arguments
    ///
    /// * `min_control_temperature`
    /// * `temperature_unit`
    pub async fn set_min_control_temperature(
        &self,
        min_control_temperature: u8,
        temperature_unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new()
                .min_control_temperature(min_control_temperature, temperature_unit)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.hub_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets the *maximum control temperature*.
    ///
    /// # Arguments
    ///
    /// * `max_control_temperature`
    /// * `temperature_unit`
    pub async fn set_max_control_temperature(
        &self,
        max_control_temperature: u8,
        temperature_unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new()
                .max_control_temperature(max_control_temperature, temperature_unit)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.hub_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets *frost protection* on the device to *on* or *off*.
    ///     
    /// # Arguments
    ///
    /// * `frost_protection_on` - true/false
    pub async fn set_frost_protection(&self, frost_protection_on: bool) -> Result<(), Error> {
        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new().frost_protection_on(frost_protection_on)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.hub_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets *child protection* on the device to *on* or *off*.
    ///     
    /// # Arguments
    ///
    /// * `child_protection_on` - true/false
    pub async fn set_child_protection(&self, child_protection_on: bool) -> Result<(), Error> {
        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new().child_protection(child_protection_on)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.hub_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets the *temperature offset*.
    ///
    /// # Arguments
    ///
    /// * `temperature_offset` - between -10 and 10
    /// * `temperature_unit`
    pub async fn set_temperature_offset(
        &self,
        temperature_offset: i8,
        temperature_unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new()
                .temperature_offset(temperature_offset, temperature_unit)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.hub_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }
}
