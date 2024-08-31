use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tapo::responses::{ChildDeviceHubResult, DeviceInfoHubResult};
use tapo::HubHandler;
use tokio::sync::RwLock;

use crate::call_handler_method;
use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "HubHandler")]
pub struct PyHubHandler {
    handler: Arc<RwLock<HubHandler>>,
}

impl PyHubHandler {
    pub fn new(handler: HubHandler) -> Self {
        Self {
            handler: Arc::new(RwLock::new(handler)),
        }
    }
}

#[pymethods]
impl PyHubHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            HubHandler::refresh_session,
            discard_result
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoHubResult> {
        let handler = self.handler.clone();
        call_handler_method!(handler.read().await.deref(), HubHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_device_info_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_list(&self) -> PyResult<Py<PyList>> {
        let handler = self.handler.clone();
        let children = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_child_device_list
        )?;

        let results = Python::with_gil(|py| {
            let results = PyList::empty_bound(py);

            for child in children {
                match child {
                    ChildDeviceHubResult::KE100(x) => {
                        let _ = results.append(x.into_py(py));
                    }
                    ChildDeviceHubResult::S200B(x) => {
                        let _ = results.append(x.into_py(py));
                    }
                    ChildDeviceHubResult::T100(x) => {
                        let _ = results.append(x.into_py(py));
                    }
                    ChildDeviceHubResult::T110(x) => {
                        let _ = results.append(x.into_py(py));
                    }
                    ChildDeviceHubResult::T300(x) => {
                        let _ = results.append(x.into_py(py));
                    }
                    ChildDeviceHubResult::T310(x) => {
                        let _ = results.append(x.into_py(py));
                    }
                    ChildDeviceHubResult::T315(x) => {
                        let _ = results.append(x.into_py(py));
                    }
                    _ => {
                        let _ = results.append(py.None());
                    }
                }
            }

            results.into()
        });

        Ok(results)
    }

    pub async fn get_child_device_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_child_device_list_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_child_device_component_list_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            HubHandler::get_child_device_component_list_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }
}
