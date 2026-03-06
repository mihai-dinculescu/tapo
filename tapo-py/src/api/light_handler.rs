use std::ops::Deref;

use pyo3::prelude::*;
use tapo::LightHandler;
use tapo::responses::{DeviceInfoLightResult, DeviceUsageEnergyMonitoringResult};

use crate::call_handler_method;

py_handler! {
    PyLightHandler(LightHandler, DeviceInfoLightResult),
    py_name = "LightHandler",
    on_off,
    device_management,
    device_usage = DeviceUsageEnergyMonitoringResult,
}

#[pymethods]
impl PyLightHandler {
    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            LightHandler::set_brightness,
            brightness
        )
    }
}
