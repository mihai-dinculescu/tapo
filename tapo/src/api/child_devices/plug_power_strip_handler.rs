use crate::api::PowerStripHandler;
use crate::error::{Error, TapoResponseError};
use crate::requests::{EmptyParams, GenericSetDeviceInfoParams, TapoParams, TapoRequest};
use crate::responses::{DecodableResultExt, PlugPowerStripResult};

/// Handler for the [P300](https://www.tapo.com/en/search/?q=P300) child plugs.
pub struct PlugPowerStripHandler<'h> {
    power_strip_handler: &'h PowerStripHandler,
    device_id: String,
}

impl<'h> PlugPowerStripHandler<'h> {
    pub(crate) fn new(power_strip_handler: &'h PowerStripHandler, device_id: String) -> Self {
        Self {
            power_strip_handler,
            device_id,
        }
    }

    /// Returns *device info* as [`PlugPowerStripResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self) -> Result<PlugPowerStripResult, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.power_strip_handler
            .control_child::<PlugPowerStripResult>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.decode())?
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.power_strip_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// Turns *on* the device.
    pub async fn on(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.power_strip_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> Result<(), Error> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        let request = TapoRequest::SetDeviceInfo(Box::new(TapoParams::new(json)));

        self.power_strip_handler
            .control_child::<serde_json::Value>(self.device_id.clone(), request)
            .await?;

        Ok(())
    }
}
