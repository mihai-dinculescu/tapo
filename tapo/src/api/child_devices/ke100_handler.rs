use crate::api::HubHandler;
use crate::error::{Error,TapoResponseError};
use crate::requests::{EmptyParams, TapoParams, TapoRequest, TrvSetDeviceInfoParams};
use crate::responses::KE100Result;

/// Handler for the [KE100] device.
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
            .control_child(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
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
    /// * `target_temperature` - between 5 and 30
    pub async fn set_temperature(&self, target_temperature: u8) -> Result<Option<KE100Result>, Error> {
        let json = serde_json::to_value(TrvSetDeviceInfoParams::new().target_temp(target_temperature)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        let result = self.hub_handler
            .control_child(self.device_id.clone(), request)
            .await;

        if result.is_err() {
            return result;
        }

        Ok(result.unwrap())
    }

    /// Sets frost protection on the device to *on* or *off*.
    ///     
    /// # Arguments
    ///
    /// * `frost_protection_on` - true/false
    pub async fn set_frost_protection(&self, frost_protection_on: bool) -> Result<Option<KE100Result>, Error> {
        let json = serde_json::to_value(TrvSetDeviceInfoParams::new().frost_protection_on(frost_protection_on)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));
    
        let result = self.hub_handler
            .control_child(self.device_id.clone(), request)
            .await;

        if result.is_err() {
            return result;
        }
        
        Ok(result.unwrap())
    }

    /// Sets child protection on the device to *on* or *off*.
    ///     
    /// # Arguments
    ///
    /// * `child_protection_on` - true/false
    pub async fn set_child_protection(&self, child_protection_on: bool) -> Result<Option<KE100Result>, Error> {
        let json = serde_json::to_value(TrvSetDeviceInfoParams::new().child_protection(child_protection_on)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));
    
        let result = self.hub_handler
            .control_child(self.device_id.clone(), request)
            .await;

        if result.is_err() {
            return result;
        }
        
        Ok(result.unwrap())
    }

    /// Sets the *temperature offset*.
    ///
    /// # Arguments
    ///
    /// * `temp_offset` - between 5 and 30
    pub async fn set_temp_offset(&self, temp_offset: i8) -> Result<Option<KE100Result>, Error> {
        let json = serde_json::to_value(TrvSetDeviceInfoParams::new().temp_offset(temp_offset)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        let result = self.hub_handler
            .control_child(self.device_id.clone(), request)
            .await;

        if result.is_err() {
            return result;
        }

        Ok(result.unwrap())
    }

}
