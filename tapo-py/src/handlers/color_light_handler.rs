use std::sync::Arc;

use pyo3::prelude::*;
use tapo::{requests::Color, ColorLightHandler};
use tokio::sync::Mutex;

use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "ColorColorLightHandler")]
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
