use std::sync::Arc;

use chrono::NaiveDate;
use pyo3::prelude::*;
use tapo::requests::EnergyDataInterval;
use tapo::PlugEnergyMonitoringHandler;
use tokio::sync::Mutex;

use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "EnergyDataInterval")]
pub enum PyEnergyDataInterval {
    Hourly,
    Daily,
    Monthly,
}

#[derive(Clone)]
#[pyclass(name = "PlugEnergyMonitoringHandler")]
pub struct PyPlugEnergyMonitoringHandler {
    handler: Arc<Mutex<PlugEnergyMonitoringHandler>>,
}

impl PyPlugEnergyMonitoringHandler {
    pub fn new(handler: PlugEnergyMonitoringHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyPlugEnergyMonitoringHandler {
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

    pub fn get_current_power<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = handler
                .lock()
                .await
                .get_current_power()
                .await
                .map_err(ErrorWrapper)?;
            Ok(result)
        })
    }

    pub fn get_energy_usage<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = handler
                .lock()
                .await
                .get_energy_usage()
                .await
                .map_err(ErrorWrapper)?;
            Ok(result)
        })
    }

    pub fn get_energy_data<'a>(
        &'a self,
        py: Python<'a>,
        interval: PyEnergyDataInterval,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> PyResult<&'a PyAny> {
        let handler = self.handler.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let interval = match interval {
                PyEnergyDataInterval::Hourly => EnergyDataInterval::Hourly {
                    start_date,
                    end_date: end_date.unwrap_or(start_date),
                },
                PyEnergyDataInterval::Daily => EnergyDataInterval::Daily { start_date },
                PyEnergyDataInterval::Monthly => EnergyDataInterval::Monthly { start_date },
            };

            let result = handler
                .lock()
                .await
                .get_energy_data(interval)
                .await
                .map_err(ErrorWrapper)?;
            Ok(result)
        })
    }
}
