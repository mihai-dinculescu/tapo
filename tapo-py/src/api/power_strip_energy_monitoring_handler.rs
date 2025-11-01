use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tapo::responses::DeviceInfoPowerStripResult;
use tapo::{DeviceManagementExt as _, Error, Plug, PowerStripEnergyMonitoringHandler};
use tokio::sync::RwLock;

use crate::api::PyPowerStripPlugEnergyMonitoringHandler;
use crate::call_handler_method;
use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "PowerStripEnergyMonitoringHandler")]
pub struct PyPowerStripEnergyMonitoringHandler {
    inner: Arc<RwLock<PowerStripEnergyMonitoringHandler>>,
}

impl PyPowerStripEnergyMonitoringHandler {
    pub fn new(handler: PowerStripEnergyMonitoringHandler) -> Self {
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
impl PyPowerStripEnergyMonitoringHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            PowerStripEnergyMonitoringHandler::refresh_session,
            discard_result
        )
    }

    pub async fn device_reboot(&self, delay_s: u16) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::device_reboot,
            delay_s
        )
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::device_reset,
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoPowerStripResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::get_device_info_json
        )?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_list(&self) -> PyResult<Py<PyList>> {
        let handler = self.inner.clone();
        let children = call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::get_child_device_list
        )?;

        Python::attach(|py| {
            let results = PyList::empty(py);

            for child in children {
                results.append(child.into_pyobject(py)?)?;
            }

            Ok(results.into())
        })
    }

    pub async fn get_child_device_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::get_child_device_list_json
        )?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_component_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::get_child_device_component_list_json
        )?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    #[pyo3(signature = (device_id=None, nickname=None, position=None))]
    pub async fn plug(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
        position: Option<u8>,
    ) -> PyResult<PyPowerStripPlugEnergyMonitoringHandler> {
        let handler = self.inner.clone();
        let identifier =
            PyPowerStripEnergyMonitoringHandler::parse_identifier(device_id, nickname, position)?;

        let child_handler = call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::plug,
            identifier
        )?;
        Ok(PyPowerStripPlugEnergyMonitoringHandler::new(child_handler))
    }
}
