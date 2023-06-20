use std::sync::Arc;

use pyo3::prelude::*;
use tapo::EnergyMonitoringPlugHandler;
use tokio::sync::Mutex;

use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "EnergyMonitoringPlugHandler")]
pub struct PyEnergyMonitoringPlugHandler {
    handler: Arc<Mutex<EnergyMonitoringPlugHandler>>,
}

impl PyEnergyMonitoringPlugHandler {
    pub fn new(handler: EnergyMonitoringPlugHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyEnergyMonitoringPlugHandler {
    pub fn login<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler.lock().await.login().await.map_err(ErrorWrapper)?;
            Ok(())
        })
    }

    pub fn on<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler.lock().await.on().await.map_err(ErrorWrapper)?;
            Ok(())
        })
    }

    pub fn off<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler.lock().await.off().await.map_err(ErrorWrapper)?;
            Ok(())
        })
    }

    pub fn get_device_info<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let device_info = handler
                .lock()
                .await
                .get_device_info()
                .await
                .map_err(ErrorWrapper)?;
            Ok(device_info)
        })
    }

    pub fn get_device_usage<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let device_info = handler
                .lock()
                .await
                .get_device_usage()
                .await
                .map_err(ErrorWrapper)?;
            Ok(device_info)
        })
    }
}
