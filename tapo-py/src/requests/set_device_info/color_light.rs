use std::ops::Deref;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use tapo::requests::{Color, ColorLightSetDeviceInfoParams};

use crate::api::{
    PyColorLightHandler, PyHandlerExt, PyRgbLightStripHandler, PyRgbicLightStripHandler,
};
use crate::errors::ErrorWrapper;
use crate::runtime::tokio;

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

    async fn _send_to_inner_handler(&self, handler: impl PyHandlerExt) -> PyResult<()> {
        let params = self.params.clone();
        let handler = handler.get_inner_handler();

        tokio()
            .spawn(async move {
                let handler_lock = handler.read().await;

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

    async fn send(&self, handler: Py<PyAny>) -> PyResult<()> {
        if let Some(handler) =
            Python::with_gil(|py| handler.extract::<PyColorLightHandler>(py).ok())
        {
            return self._send_to_inner_handler(handler).await;
        }

        if let Some(handler) =
            Python::with_gil(|py| handler.extract::<PyRgbLightStripHandler>(py).ok())
        {
            return self._send_to_inner_handler(handler).await;
        }

        if let Some(handler) =
            Python::with_gil(|py| handler.extract::<PyRgbicLightStripHandler>(py).ok())
        {
            return self._send_to_inner_handler(handler).await;
        }

        Err(PyErr::new::<PyTypeError, _>(
            "Invalid handler type. Must be one of `PyColorLightHandler`, `PyRgbLightStripHandler` or `PyRgbicLightStripHandler`",
        ))
    }
}
