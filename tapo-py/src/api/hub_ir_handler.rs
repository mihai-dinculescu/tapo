use std::ops::Deref;
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyList;
use tapo::HubIrHandler;
use tapo::responses::ChildDeviceHubIrResult;
use tokio::sync::RwLock;

use crate::api::{PyHubHandler, PyIrRemoteHandler};
use crate::call_handler_method;

/// Python `HubIrHandler`, a subclass of `HubHandler`.
///
/// Everything it has in common with the H100 is inherited from `HubHandler`, whose
/// handler shares this handler's authenticated session. Only the methods that
/// involve IR remotes are implemented here.
#[pyclass(extends = PyHubHandler, name = "HubIrHandler")]
pub struct PyHubIrHandler {
    inner: Arc<RwLock<HubIrHandler>>,
}

impl PyHubIrHandler {
    pub fn new(handler: HubIrHandler) -> PyClassInitializer<Self> {
        let base = PyHubHandler::new(handler.to_hub_handler());

        PyClassInitializer::from(base).add_subclass(Self {
            inner: Arc::new(RwLock::new(handler)),
        })
    }

    pub fn into_py(handler: HubIrHandler) -> PyResult<Py<Self>> {
        Python::attach(|py| Py::new(py, Self::new(handler)))
    }
}

#[pymethods]
impl PyHubIrHandler {
    pub async fn get_child_device_list(&self) -> PyResult<Py<PyList>> {
        let handler = self.inner.clone();
        let children = call_handler_method!(
            handler.read().await.deref(),
            HubIrHandler::get_child_device_list
        )?;

        Python::attach(|py| {
            let results = PyList::empty(py);

            for child in children {
                match child {
                    ChildDeviceHubIrResult::IrRemote(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::KE100(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::S200(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::S210(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::T100(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::T110(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::T300(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::T31X(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                    ChildDeviceHubIrResult::Other(device) => {
                        results.append(device.into_pyobject(py)?)?;
                    }
                }
            }

            Ok(results.into())
        })
    }

    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn ir_remote(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyIrRemoteHandler> {
        let handler = self.inner.clone();
        let identifier = PyHubHandler::parse_identifier(device_id, nickname)?;

        let child_handler = call_handler_method!(
            handler.read().await.deref(),
            HubIrHandler::ir_remote,
            identifier
        )?;
        Ok(PyIrRemoteHandler::new(child_handler))
    }

    pub async fn ir_remote_unchecked(&self, device_id: String) -> PyResult<PyIrRemoteHandler> {
        let handler = self.inner.clone();
        let child = handler.read().await.ir_remote_unchecked(device_id);
        Ok(PyIrRemoteHandler::new(child))
    }
}
