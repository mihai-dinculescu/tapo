use crate::error::{Error, TapoResponseError};
use crate::requests::{EmptyParams, TapoParams, TapoRequest};
use crate::responses::{T31XResult, TemperatureHumidityRecords, TemperatureHumidityRecordsRaw};

tapo_child_handler! {
    /// Handler for the [T310](https://www.tapo.com/en/search/?q=T310) and [T315](https://www.tapo.com/en/search/?q=T315) devices.
    T31XHandler(T31XResult),
}

impl T31XHandler {
    /// Returns *temperature and humidity records* from the last 24 hours at 15 minute intervals as [`TemperatureHumidityRecords`].
    pub async fn get_temperature_humidity_records(
        &self,
    ) -> Result<TemperatureHumidityRecords, Error> {
        let request =
            TapoRequest::GetTemperatureHumidityRecords(Box::new(TapoParams::new(EmptyParams)));

        let result = self
            .client
            .read()
            .await
            .control_child::<TemperatureHumidityRecordsRaw>(self.device_id.clone(), request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(result.try_into()?)
    }
}
