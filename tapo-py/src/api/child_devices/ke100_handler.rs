use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::responses::{KE100Result, TemperatureUnitKE100};
use tapo::KE100Handler;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "KE100Handler")]
pub struct PyKE100Handler {
    inner: Arc<KE100Handler>,
}

impl PyKE100Handler {
    pub fn new(handler: KE100Handler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyKE100Handler {
    pub async fn get_device_info(&self) -> PyResult<KE100Result> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), KE100Handler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), KE100Handler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn set_child_protection(&self, on: bool) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), KE100Handler::set_child_protection, on)
    }

    pub async fn set_frost_protection(&self, on: bool) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), KE100Handler::set_frost_protection, on)
    }

    pub async fn set_max_control_temperature(
        &self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            KE100Handler::set_max_control_temperature,
            value,
            unit
        )
    }

    pub async fn set_min_control_temperature(
        &self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            KE100Handler::set_min_control_temperature,
            value,
            unit
        )
    }

    pub async fn set_target_temperature(
        &self,
        value: u8,
        unit: TemperatureUnitKE100,
    ) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            KE100Handler::set_target_temperature,
            value,
            unit
        )
    }

    pub async fn set_temperature_offset(
        &self,
        value: i8,
        unit: TemperatureUnitKE100,
    ) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            KE100Handler::set_temperature_offset,
            value,
            unit
        )
    }
}
