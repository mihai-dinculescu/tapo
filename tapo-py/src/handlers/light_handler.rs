use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::{DeviceInfoLightResult, DeviceUsageEnergyMonitoringResult};
use tapo::LightHandler;
use tokio::sync::Mutex;

use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "LightHandler")]
pub struct PyLightHandler {
    handler: Arc<Mutex<LightHandler>>,
}

impl PyLightHandler {
    pub fn new(handler: LightHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyLightHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler
            .lock()
            .await
            .refresh_session()
            .await
            .map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler.lock().await.on().await.map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler.lock().await.off().await.map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler
            .lock()
            .await
            .device_reset()
            .await
            .map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoLightResult> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_device_info()
            .await
            .map_err(ErrorWrapper)?;
        Ok(result)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_device_info_json()
            .await
            .map_err(ErrorWrapper)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_device_usage()
            .await
            .map_err(ErrorWrapper)?;
        Ok(result)
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.handler.clone();
        handler
            .lock()
            .await
            .set_brightness(brightness)
            .await
            .map_err(ErrorWrapper)?;
        Ok(())
    }
}
