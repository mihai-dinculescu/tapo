use std::ops::Deref;

use pyo3::prelude::*;
use tapo::KE100Handler;
use tapo::responses::{KE100Result, TemperatureUnitKE100};

use crate::call_handler_method;

py_child_handler! {
    PyKE100Handler(KE100Handler, KE100Result),
    py_name = "KE100Handler",
}

#[pymethods]
impl PyKE100Handler {
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
