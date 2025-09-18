use std::sync::Arc;

use pyo3::prelude::*;
use tapo::{DeviceDiscovery, StreamExt as _};
use tokio::sync::Mutex;

use super::{PyMaybeDiscoveryResult, convert_result_to_maybe_py};

#[derive(Clone)]
#[pyclass(name = "DeviceDiscovery")]
pub struct PyDeviceDiscovery {
    pub inner: Arc<Mutex<DeviceDiscovery>>,
}

impl PyDeviceDiscovery {
    pub fn new(inner: DeviceDiscovery) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

#[pymethods]
impl PyDeviceDiscovery {
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<PyDeviceDiscoveryIter> {
        Ok(PyDeviceDiscoveryIter {
            inner: (*slf).inner.clone(),
        })
    }
    fn __aiter__(slf: PyRef<'_, Self>) -> PyResult<PyDeviceDiscoveryIter> {
        Ok(PyDeviceDiscoveryIter {
            inner: (*slf).inner.clone(),
        })
    }
}

#[pyclass(name = "DeviceDiscoveryIter")]
pub struct PyDeviceDiscoveryIter {
    pub inner: Arc<Mutex<DeviceDiscovery>>,
}

#[pymethods]
impl PyDeviceDiscoveryIter {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(slf: PyRefMut<'_, Self>) -> PyResult<Option<PyMaybeDiscoveryResult>> {
        let inner = (*slf).inner.clone();

        let result = Python::attach(|py| {
            py.detach(|| {
                crate::runtime::tokio().block_on(async {
                    let mut guard = inner.lock().await;
                    guard.next().await
                })
            })
        });

        if let Some(result) = result {
            let result_maybe_py = convert_result_to_maybe_py(result)?;
            Ok(Some(result_maybe_py))
        } else {
            Ok(None)
        }
    }

    fn __anext__<'py>(slf: PyRefMut<'_, Self>, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = (*slf).inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = inner.lock().await;
            let result = guard.next().await;

            match result {
                Some(result) => convert_result_to_maybe_py(result),
                None => Err(PyErr::new::<pyo3::exceptions::PyStopAsyncIteration, _>(
                    "No more devices found",
                )),
            }
        })
    }
}
