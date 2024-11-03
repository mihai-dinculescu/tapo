use std::sync::Arc;

use tokio::sync::RwLock;

use crate::api::ApiClient;
use crate::error::{Error, TapoResponseError};
use crate::requests::TemperatureUnitKE100;
use crate::requests::{EmptyParams, TapoParams, TapoRequest, TrvSetDeviceInfoParams};
use crate::responses::{DecodableResultExt, KE100Result};

/// Handler for the [KE100](https://www.tp-link.com/en/search/?q=KE100) devices.
pub struct KE100Handler {
    client: Arc<RwLock<ApiClient>>,
    device_id: String,
}

impl KE100Handler {
    pub(crate) fn new(client: Arc<RwLock<ApiClient>>, device_id: String) -> Self {
        Self { client, device_id }
    }

    /// Returns *device info* as [`KE100Result`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self) -> Result<KE100Result, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.client
            .read()
            .await
            .control_child::<KE100Result>(self.device_id.clone(), request)
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

    /// Sets *child protection* on the device to *on* or *off*.
    ///     
    /// # Arguments
    ///
    /// * `on`
    pub async fn set_child_protection(&self, on: bool) -> Result<(), Error> {
        let json = serde_json::to_value(TrvSetDeviceInfoParams::new().child_protection(on)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets *frost protection* on the device to *on* or *off*.
    ///     
    /// # Arguments
    ///
    /// * `on`
    pub async fn set_frost_protection(&self, on: bool) -> Result<(), Error> {
        let json = serde_json::to_value(TrvSetDeviceInfoParams::new().frost_protection_on(on)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets the *maximum control temperature*.
    ///
    /// # Arguments
    ///
    /// * `value`
    /// * `unit`
    pub async fn set_max_control_temperature(
        &self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new().max_control_temperature(value, unit)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets the *minimum control temperature*.
    ///
    /// # Arguments
    ///
    /// * `value`
    /// * `unit`
    pub async fn set_min_control_temperature(
        &self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let json = serde_json::to_value(
            TrvSetDeviceInfoParams::new().min_control_temperature(value, unit)?,
        )?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets the *target temperature*.
    ///
    /// # Arguments
    ///
    /// * `value` - between `min_control_temperature` and `max_control_temperature`
    /// * `unit`
    pub async fn set_target_temperature(
        &self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let device_info = self.get_device_info().await?;

        if value < device_info.min_control_temperature
            || value > device_info.max_control_temperature
        {
            return Err(Error::Validation {
                field: "target_temperature".to_string(),
                message: format!(
                    "Target temperature must be between {} (min_control_temperature) and {} (max_control_temperature)",
                    device_info.min_control_temperature, device_info.max_control_temperature
                ),
            });
        }

        let json =
            serde_json::to_value(TrvSetDeviceInfoParams::new().target_temperature(value, unit)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Sets the *temperature offset*.
    ///
    /// # Arguments
    ///
    /// * `value` - between -10 and 10
    /// * `unit`
    pub async fn set_temperature_offset(
        &self,
        value: i8,
        unit: TemperatureUnitKE100,
    ) -> Result<(), Error> {
        let json =
            serde_json::to_value(TrvSetDeviceInfoParams::new().temperature_offset(value, unit)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.client
            .read()
            .await
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }
}
