use std::ops::Deref;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tapo::responses::{ChildDeviceComponentList, DeviceInfoPowerStripResult};
use tapo::{Error, Plug, PowerStripEnergyMonitoringHandler};

use crate::api::PyPowerStripPlugEnergyMonitoringHandler;
use crate::call_handler_method;

py_handler! {
    PyPowerStripEnergyMonitoringHandler(PowerStripEnergyMonitoringHandler, DeviceInfoPowerStripResult),
    py_name = "PowerStripEnergyMonitoringHandler",
    device_management,
}

impl PyPowerStripEnergyMonitoringHandler {
    fn parse_identifier(
        device_id: Option<String>,
        nickname: Option<String>,
        position: Option<u8>,
    ) -> PyResult<Plug> {
        match (device_id, nickname, position) {
            (Some(device_id), _, _) => Ok(Plug::ByDeviceId(device_id)),
            (None, Some(nickname), _) => Ok(Plug::ByNickname(nickname)),
            (None, None, Some(position)) => Ok(Plug::ByPosition(position)),
            _ => Err(Error::Validation {
                field: "identifier".to_string(),
                message: "Either a device_id, nickname, or position must be provided".to_string(),
            }
            .into()),
        }
    }
}

#[pymethods]
impl PyPowerStripEnergyMonitoringHandler {
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

    pub async fn get_child_device_component_list(&self) -> PyResult<Vec<ChildDeviceComponentList>> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PowerStripEnergyMonitoringHandler::get_child_device_component_list
        )
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
