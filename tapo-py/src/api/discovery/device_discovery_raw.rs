use std::sync::Arc;

use pyo3::prelude::*;
use tapo::{DeviceDiscoveryRaw, StreamExt as _};
use tokio::sync::Mutex;

use super::{PyMaybeDiscoveryRawResult, convert_raw_result_to_maybe_py};

#[derive(Clone)]
#[pyclass(from_py_object, name = "DeviceDiscoveryRaw")]
pub struct PyDeviceDiscoveryRaw {
    pub inner: Arc<Mutex<DeviceDiscoveryRaw>>,
}

impl PyDeviceDiscoveryRaw {
    pub fn new(inner: DeviceDiscoveryRaw) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

#[pymethods]
impl PyDeviceDiscoveryRaw {
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<PyDeviceDiscoveryRawIter> {
        Ok(PyDeviceDiscoveryRawIter {
            inner: (*slf).inner.clone(),
        })
    }
    fn __aiter__(slf: PyRef<'_, Self>) -> PyResult<PyDeviceDiscoveryRawIter> {
        Ok(PyDeviceDiscoveryRawIter {
            inner: (*slf).inner.clone(),
        })
    }
}

#[pyclass(name = "DeviceDiscoveryRawIter")]
pub struct PyDeviceDiscoveryRawIter {
    pub inner: Arc<Mutex<DeviceDiscoveryRaw>>,
}

#[pymethods]
impl PyDeviceDiscoveryRawIter {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(slf: PyRefMut<'_, Self>) -> PyResult<Option<PyMaybeDiscoveryRawResult>> {
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
            let result_maybe_py = convert_raw_result_to_maybe_py(result)?;
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
                Some(result) => convert_raw_result_to_maybe_py(result),
                None => Err(PyErr::new::<pyo3::exceptions::PyStopAsyncIteration, _>(
                    "No more devices found",
                )),
            }
        })
    }
}
