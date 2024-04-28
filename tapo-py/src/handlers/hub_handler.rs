use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyList;
use tapo::responses::ChildDeviceHubResult;
use tapo::HubHandler;
use tokio::sync::Mutex;

use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "HubHandler")]
pub struct PyHubHandler {
    handler: Arc<Mutex<HubHandler>>,
}

impl PyHubHandler {
    pub fn new(handler: HubHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyHubHandler {
    pub fn refresh_session<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler
                .lock()
                .await
                .refresh_session()
                .await
                .map_err(ErrorWrapper)?;
            Ok(())
        })
    }

    pub fn get_device_info<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = handler
                .lock()
                .await
                .get_device_info()
                .await
                .map_err(ErrorWrapper)?;
            Ok(result)
        })
    }

    pub fn get_device_info_json<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = handler
                .lock()
                .await
                .get_device_info_json()
                .await
                .map_err(ErrorWrapper)?;

            Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
        })
    }

    pub fn get_child_device_list<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();

        pyo3_asyncio::tokio::future_into_py::<_, Py<PyList>>(py, async move {
            let children = handler
                .lock()
                .await
                .get_child_device_list()
                .await
                .map_err(ErrorWrapper)?;

            Ok(Python::with_gil(|py| {
                let results = PyList::empty(py);

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

                results.into_py(py)
            }))
        })
    }

    pub fn get_child_device_list_json<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = handler
                .lock()
                .await
                .get_child_device_list_json()
                .await
                .map_err(ErrorWrapper)?;

            Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
        })
    }

    pub fn get_child_device_component_list_json<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = handler
                .lock()
                .await
                .get_child_device_component_list_json()
                .await
                .map_err(ErrorWrapper)?;

            Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
        })
    }
}
