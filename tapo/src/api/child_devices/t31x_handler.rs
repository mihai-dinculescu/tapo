use crate::api::HubHandler;
use crate::error::{Error, TapoResponseError};
use crate::requests::{EmptyParams, TapoParams, TapoRequest};
use crate::responses::{
    DecodableResultExt, T31XResult, TemperatureHumidityRecords, TemperatureHumidityRecordsRaw,
};

/// Handler for the [T310](https://www.tapo.com/en/search/?q=T310) and [T315](https://www.tapo.com/en/search/?q=T315) devices.
pub struct T31XHandler<'h> {
    hub_handler: &'h HubHandler,
    device_id: String,
}

impl<'h> T31XHandler<'h> {
    pub(crate) fn new(hub_handler: &'h HubHandler, device_id: String) -> Self {
        Self {
            hub_handler,
            device_id,
        }
    }

    /// Returns *device info* as [`T31XResult`].
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    pub async fn get_device_info(&self) -> Result<T31XResult, Error> {
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.hub_handler
            .control_child::<T31XResult>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.decode())?
    }

    /// Returns *temperature and humidity records* from the last 24 hours at 15 minute intervals as [`TemperatureHumidityRecords`].
    pub async fn get_temperature_humidity_records(
        &self,
    ) -> Result<TemperatureHumidityRecords, Error> {
        let request =
            TapoRequest::GetTemperatureHumidityRecords(Box::new(TapoParams::new(EmptyParams)));

        let result = self
            .hub_handler
            .control_child::<TemperatureHumidityRecordsRaw>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(result.try_into()?)
    }
}
