use crate::api::PowerStripHandler;
use crate::error::{Error, TapoResponseError};
use crate::requests::{EmptyParams, GenericSetDeviceInfoParams, TapoParams, TapoRequest};
use crate::responses::{DecodableResultExt, P300ChildResult};


/// Handler for the [P300](https://www.tapo.com/en/search/?q=T100) child devices.
pub struct P300ChildHandler<'h> {
    power_strip_handler: &'h PowerStripHandler,
    device_id: String,
}

impl<'h> P300ChildHandler<'h> {
    pub(crate) fn new(power_strip_handler: &'h PowerStripHandler, device_id: String) -> Self {
        Self {
            power_strip_handler,
            device_id,
        }
    }

    /// Returns *device info* as [`P300ChildResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self) -> Result<P300ChildResult, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.power_strip_handler
            .control_child::<P300ChildResult>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.decode())?
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
