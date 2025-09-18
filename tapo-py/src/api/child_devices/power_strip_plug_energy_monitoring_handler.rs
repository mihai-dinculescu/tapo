use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::PowerStripPlugEnergyMonitoringHandler;
use tapo::responses::PowerStripPlugEnergyMonitoringResult;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "PowerStripPlugEnergyMonitoringHandler")]
pub struct PyPowerStripPlugEnergyMonitoringHandler {
    inner: Arc<PowerStripPlugEnergyMonitoringHandler>,
}

impl PyPowerStripPlugEnergyMonitoringHandler {
    pub fn new(handler: PowerStripPlugEnergyMonitoringHandler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyPowerStripPlugEnergyMonitoringHandler {
    pub async fn get_device_info(&self) -> PyResult<PowerStripPlugEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_device_info_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), PowerStripPlugEnergyMonitoringHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), PowerStripPlugEnergyMonitoringHandler::off)
    }
}
