use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::requests::{Color, ColorLightSetDeviceInfoParams};
use tapo::responses::{DeviceInfoColorLightResult, DeviceUsageEnergyMonitoringResult};
use tapo::ColorLightHandler;
use tokio::sync::RwLock;

use crate::call_handler_method;
use crate::errors::ErrorWrapper;
use crate::runtime::tokio;

#[derive(Clone)]
#[pyclass(name = "ColorLightHandler")]
pub struct PyColorLightHandler {
    handler: Arc<RwLock<ColorLightHandler>>,
}

impl PyColorLightHandler {
    pub fn new(handler: ColorLightHandler) -> Self {
        Self {
            handler: Arc::new(RwLock::new(handler)),
        }
    }
}

#[pymethods]
impl PyColorLightHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            ColorLightHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(handler.read().await.deref(), ColorLightHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(handler.read().await.deref(), ColorLightHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::device_reset
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoColorLightResult> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::get_device_info_json,
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::get_device_usage
        )
    }

    pub fn set(&self) -> PyColorLightSetDeviceInfoParams {
        PyColorLightSetDeviceInfoParams::new()
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_brightness,
            brightness
        )
    }

    pub async fn set_color(&self, color: Color) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_color,
            color
        )
    }

    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_hue_saturation,
            hue,
            saturation
        )
    }

    pub async fn set_color_temperature(&self, color_temperature: u16) -> PyResult<()> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_color_temperature,
            color_temperature
        )
    }
}

#[derive(Clone)]
#[pyclass(name = "LightSetDeviceInfoParams")]
pub struct PyColorLightSetDeviceInfoParams {
    params: ColorLightSetDeviceInfoParams,
}

impl PyColorLightSetDeviceInfoParams {
    pub(crate) fn new() -> Self {
        Self {
            params: ColorLightSetDeviceInfoParams::new(),
        }
    }
}

#[pymethods]
impl PyColorLightSetDeviceInfoParams {
    pub fn on(&self) -> Self {
        Self {
            params: self.params.clone().on(),
        }
    }

    pub fn off(&self) -> Self {
        Self {
            params: self.params.clone().off(),
        }
    }

    pub fn brightness(&self, brightness: u8) -> Self {
        Self {
            params: self.params.clone().brightness(brightness),
        }
    }

    pub fn color(&self, color: Color) -> Self {
        Self {
            params: self.params.clone().color(color),
        }
    }

    pub fn hue_saturation(&self, hue: u16, saturation: u8) -> Self {
        Self {
            params: self.params.clone().hue_saturation(hue, saturation),
        }
    }

    pub fn color_temperature(&self, color_temperature: u16) -> Self {
        Self {
            params: self.params.clone().color_temperature(color_temperature),
        }
    }

    pub async fn send(&self, handler: PyColorLightHandler) -> PyResult<()> {
        let params = self.params.clone();

        tokio()
            .spawn(async move {
                let handler_lock = handler.handler.read().await;

                params
                    .send(handler_lock.deref())
                    .await
                    .map_err(ErrorWrapper)?;

                Ok::<_, ErrorWrapper>(())
            })
            .await
            .map_err(anyhow::Error::from)
            .map_err(ErrorWrapper::from)??;

        Ok(())
    }
}
