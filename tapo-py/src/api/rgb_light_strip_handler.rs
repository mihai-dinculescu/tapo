use std::ops::Deref;

use pyo3::prelude::*;
use tapo::RgbLightStripHandler;
use tapo::requests::Color;
use tapo::responses::{DeviceInfoRgbLightStripResult, DeviceUsageEnergyMonitoringResult};

use crate::call_handler_method;
use crate::requests::PyColorLightSetDeviceInfoParams;

py_handler! {
    PyRgbLightStripHandler(RgbLightStripHandler, DeviceInfoRgbLightStripResult),
    py_name = "RgbLightStripHandler",
    on_off,
    device_management,
    device_usage = DeviceUsageEnergyMonitoringResult,
}

#[pymethods]
impl PyRgbLightStripHandler {
    pub fn set(&self) -> PyColorLightSetDeviceInfoParams {
        PyColorLightSetDeviceInfoParams::new()
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_brightness,
            brightness
        )
    }

    pub async fn set_color(&self, color: Color) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_color,
            color
        )
    }

    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_hue_saturation,
            hue,
            saturation
        )
    }

    pub async fn set_color_temperature(&self, color_temperature: u16) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_color_temperature,
            color_temperature
        )
    }
}
