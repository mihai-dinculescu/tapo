use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tapo::responses::DeviceInfoPowerStripResult;
use tapo::{Error, Plug, PowerStripHandler};
use tokio::sync::RwLock;

use crate::api::PyPowerStripPlugHandler;
use crate::call_handler_method;
use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "PowerStripHandler")]
pub struct PyPowerStripHandler {
    inner: Arc<RwLock<PowerStripHandler>>,
}

impl PyPowerStripHandler {
    pub fn new(handler: PowerStripHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }

    fn parse_identifier(
        device_id: Option<String>,
        nickname: Option<String>,
        position: Option<u8>,
    ) -> PyResult<Plug> {
        match (device_id, nickname, position) {
            (Some(device_id), _, _) => Ok(Plug::ByDeviceId(device_id)),
            (None, Some(nickname), _) => Ok(Plug::ByNickname(nickname)),
            (None, None, Some(position)) => Ok(Plug::ByPosition(position)),
            _ => Err(Into::<ErrorWrapper>::into(Error::Validation {
                field: "identifier".to_string(),
                message: "Either a device_id, nickname, or position must be provided".to_string(),
            })
            .into()),
        }
    }
}

#[pymethods]
impl PyPowerStripHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            PowerStripHandler::refresh_session,
            discard_result
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoPowerStripResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PowerStripHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PowerStripHandler::get_device_info_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_list(&self) -> PyResult<Py<PyList>> {
        let handler = self.inner.clone();
        let children = call_handler_method!(
            handler.read().await.deref(),
            PowerStripHandler::get_child_device_list
        )?;

        let results = Python::with_gil(|py| {
            let results = PyList::empty(py);

            for child in children {
                results.append(child.into_pyobject(py)?)?;
            }

            Ok(results.into())
        });

        results
    }

    pub async fn get_child_device_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PowerStripHandler::get_child_device_list_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_component_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PowerStripHandler::get_child_device_component_list_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    #[pyo3(signature = (device_id=None, nickname=None, position=None))]
    pub async fn plug(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
        position: Option<u8>,
    ) -> PyResult<PyPowerStripPlugHandler> {
        let handler = self.inner.clone();
        let identifier = PyPowerStripHandler::parse_identifier(device_id, nickname, position)?;

        let child_handler = call_handler_method!(
            handler.read().await.deref(),
            PowerStripHandler::plug,
            identifier
        )?;
        Ok(PyPowerStripPlugHandler::new(child_handler))
    }
}
