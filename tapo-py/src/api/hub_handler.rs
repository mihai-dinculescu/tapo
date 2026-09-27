use std::ops::Deref;

use pyo3::prelude::*;
use tapo::requests::{AlarmDuration, AlarmRingtone, AlarmVolume};
use tapo::responses::DeviceInfoHubResult;
use tapo::{Error, HubHandler};

use crate::call_handler_method;
use crate::requests::PyAlarmDuration;

py_handler! {
    PyHubHandler(HubHandler, DeviceInfoHubResult),
    py_name = "HubHandler",
    device_management,
}

py_hub_child_handlers!(PyHubHandler, HubHandler);

#[pymethods]
impl PyHubHandler {
    pub async fn get_supported_ringtone_list(&self) -> PyResult<Vec<String>> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_supported_ringtone_list
        )
    }

    #[pyo3(signature = (ringtone, volume, duration, seconds=None))]
    pub async fn play_alarm(
        &self,
        ringtone: AlarmRingtone,
        volume: AlarmVolume,
        duration: PyAlarmDuration,
        seconds: Option<u32>,
    ) -> PyResult<()> {
        let handler = self.inner.clone();

        let duration = match duration {
            PyAlarmDuration::Continuous => AlarmDuration::Continuous,
            PyAlarmDuration::Once => AlarmDuration::Once,
            PyAlarmDuration::Seconds => {
                if let Some(seconds) = seconds {
                    AlarmDuration::Seconds(seconds)
                } else {
                    return Err(Error::Validation {
                        field: "seconds".to_string(),
                        message:
                            "A value must be provided for seconds when duration = AlarmDuration.Seconds"
                                .to_string(),
                    }
                    .into());
                }
            }
        };

        call_handler_method!(
            handler.read().await.deref(),
            HubHandler::play_alarm,
            ringtone,
            volume,
            duration
        )
    }

    pub async fn stop_alarm(&self) -> PyResult<()> {
        let handler = self.inner.clone();

        call_handler_method!(handler.read().await.deref(), HubHandler::stop_alarm)
    }
}
