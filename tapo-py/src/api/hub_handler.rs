use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tapo::requests::{AlarmDuration, AlarmRingtone, AlarmVolume};
use tapo::responses::{ChildDeviceHubResult, DeviceInfoHubResult};
use tapo::{Error, HubDevice, HubHandler};
use tokio::sync::RwLock;

use crate::api::{
    PyKE100Handler, PyS200BHandler, PyT100Handler, PyT110Handler, PyT300Handler, PyT31XHandler,
};
use crate::call_handler_method;
use crate::errors::ErrorWrapper;
use crate::requests::PyAlarmDuration;

#[derive(Clone)]
#[pyclass(name = "HubHandler")]
pub struct PyHubHandler {
    inner: Arc<RwLock<HubHandler>>,
}

impl PyHubHandler {
    pub fn new(handler: HubHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }

    fn parse_identifier(
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<HubDevice> {
        match (device_id, nickname) {
            (Some(device_id), _) => Ok(HubDevice::ByDeviceId(device_id)),
            (None, Some(nickname)) => Ok(HubDevice::ByNickname(nickname)),
            _ => Err(Into::<ErrorWrapper>::into(Error::Validation {
                field: "identifier".to_string(),
                message: "Either a device_id or nickname must be provided".to_string(),
            })
            .into()),
        }
    }
}

#[pymethods]
impl PyHubHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            HubHandler::refresh_session,
            discard_result
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoHubResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), HubHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_device_info_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_list(&self) -> PyResult<Py<PyList>> {
        let handler = self.inner.clone();
        let children = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_child_device_list
        )?;

        let results = Python::with_gil(|py| {
            let results = PyList::empty(py);

            for child in children {
                match child {
                    ChildDeviceHubResult::KE100(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubResult::S200B(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubResult::T100(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubResult::T110(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubResult::T300(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubResult::T310(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubResult::T315(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    _ => {
                        results.append(py.None())?;
                    }
                }
            }

            Ok(results.into())
        });

        results
    }

    pub async fn get_child_device_list_json(&self, start_index: u64) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_child_device_list_json,
            start_index
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_component_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_child_device_component_list_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

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
                    return Err(Into::<ErrorWrapper>::into(Error::Validation {
                        field: "seconds".to_string(),
                        message:
                            "A value must be provided for seconds when duration = AlarmDuration.Seconds"
                                .to_string(),
                    })
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

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn ke100(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyKE100Handler> {
        let handler = self.inner.clone();
        let identifier = PyHubHandler::parse_identifier(device_id, nickname)?;

        let child_handler =
            call_handler_method!(handler.read().await.deref(), HubHandler::ke100, identifier)?;
        Ok(PyKE100Handler::new(child_handler))
    }

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn s200b(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyS200BHandler> {
        let handler = self.inner.clone();
        let identifier = PyHubHandler::parse_identifier(device_id, nickname)?;

        let child_handler =
            call_handler_method!(handler.read().await.deref(), HubHandler::s200b, identifier)?;
        Ok(PyS200BHandler::new(child_handler))
    }

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn t100(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyT100Handler> {
        let handler = self.inner.clone();
        let identifier = PyHubHandler::parse_identifier(device_id, nickname)?;

        let child_handler =
            call_handler_method!(handler.read().await.deref(), HubHandler::t100, identifier)?;
        Ok(PyT100Handler::new(child_handler))
    }

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn t110(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyT110Handler> {
        let handler = self.inner.clone();
        let identifier = PyHubHandler::parse_identifier(device_id, nickname)?;

        let child_handler =
            call_handler_method!(handler.read().await.deref(), HubHandler::t110, identifier)?;
        Ok(PyT110Handler::new(child_handler))
    }

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn t300(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyT300Handler> {
        let handler = self.inner.clone();
        let identifier = PyHubHandler::parse_identifier(device_id, nickname)?;

        let child_handler =
            call_handler_method!(handler.read().await.deref(), HubHandler::t300, identifier)?;
        Ok(PyT300Handler::new(child_handler))
    }

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn t310(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyT31XHandler> {
        let handler = self.inner.clone();
        let identifier = PyHubHandler::parse_identifier(device_id, nickname)?;

        let child_handler =
            call_handler_method!(handler.read().await.deref(), HubHandler::t310, identifier)?;
        Ok(PyT31XHandler::new(child_handler))
    }

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn t315(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyT31XHandler> {
        self.t310(device_id, nickname).await
    }
}
