use std::{ops::Deref, sync::Arc};

use pyo3::prelude::*;
use tapo::{
    requests::{Color, ColorLightSetDeviceInfoParams},
    ColorLightHandler,
};
use tokio::sync::Mutex;

use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "ColorLightHandler")]
pub struct PyColorLightHandler {
    handler: Arc<Mutex<ColorLightHandler>>,
}

impl PyColorLightHandler {
    pub fn new(handler: ColorLightHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyColorLightHandler {
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

    pub fn device_reset<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler
                .lock()
                .await
                .device_reset()
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

    pub fn get_device_usage<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = handler
                .lock()
                .await
                .get_device_usage()
                .await
                .map_err(ErrorWrapper)?;
            Ok(result)
        })
    }

    pub fn set(&self) -> PyColorLightSetDeviceInfoParams {
        PyColorLightSetDeviceInfoParams::new()
    }

    pub fn set_brightness<'a>(&'a self, py: Python<'a>, brightness: u8) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler
                .lock()
                .await
                .set_brightness(brightness)
                .await
                .map_err(ErrorWrapper)?;
            Ok(())
        })
    }

    pub fn set_color<'a>(&'a self, py: Python<'a>, color: Color) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler
                .lock()
                .await
                .set_color(color)
                .await
                .map_err(ErrorWrapper)?;
            Ok(())
        })
    }

    pub fn set_hue_saturation<'a>(
        &'a self,
        py: Python<'a>,
        hue: u16,
        saturation: u8,
    ) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler
                .lock()
                .await
                .set_hue_saturation(hue, saturation)
                .await
                .map_err(ErrorWrapper)?;
            Ok(())
        })
    }

    pub fn set_color_temperature<'a>(
        &'a self,
        py: Python<'a>,
        color_temperature: u16,
    ) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            handler
                .lock()
                .await
                .set_color_temperature(color_temperature)
                .await
                .map_err(ErrorWrapper)?;
            Ok(())
        })
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

    pub fn send<'a>(&'a self, py: Python<'a>, handler: PyColorLightHandler) -> PyResult<&'a PyAny> {
        let params = self.params.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler_lock = handler.handler.lock().await;
            params
                .send(handler_lock.deref())
                .await
                .map_err(ErrorWrapper)?;

            Ok(())
        })
    }
}
