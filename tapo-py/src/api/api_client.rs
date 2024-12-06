use pyo3::prelude::*;
use std::time::Duration;
use tapo::{
    ApiClient, ColorLightHandler, GenericDeviceHandler, HubHandler, LightHandler,
    PlugEnergyMonitoringHandler, PlugHandler, PowerStripHandler, RgbLightStripHandler,
    RgbicLightStripHandler,
};

use crate::api::{
    PyColorLightHandler, PyGenericDeviceHandler, PyHubHandler, PyLightHandler,
    PyPlugEnergyMonitoringHandler, PyPlugHandler, PyPowerStripHandler, PyRgbLightStripHandler,
    PyRgbicLightStripHandler,
};
use crate::call_handler_constructor;
use crate::errors::ErrorWrapper;

#[pyclass(name = "ApiClient")]
pub struct PyApiClient {
    client: ApiClient,
}

#[pymethods]
impl PyApiClient {
    #[new]
    #[pyo3(signature = (tapo_username, tapo_password, timeout_s=None))]
    pub fn new(
        tapo_username: String,
        tapo_password: String,
        timeout_s: Option<u64>,
    ) -> Result<Self, ErrorWrapper> {
        let client = match timeout_s {
            Some(timeout_s) => ApiClient::new(tapo_username, tapo_password)
                .with_timeout(Duration::from_secs(timeout_s)),
            None => ApiClient::new(tapo_username, tapo_password),
        };

        Ok(Self { client })
    }

    pub async fn generic_device(&self, ip_address: String) -> PyResult<PyGenericDeviceHandler> {
        let handler: GenericDeviceHandler =
            call_handler_constructor!(self, tapo::ApiClient::generic_device, ip_address);
        Ok(PyGenericDeviceHandler::new(handler))
    }

    pub async fn l510(&self, ip_address: String) -> PyResult<PyLightHandler> {
        let handler: LightHandler =
            call_handler_constructor!(self, tapo::ApiClient::l510, ip_address);
        Ok(PyLightHandler::new(handler))
    }

    pub async fn l520(&self, ip_address: String) -> PyResult<PyLightHandler> {
        let handler: LightHandler =
            call_handler_constructor!(self, tapo::ApiClient::l520, ip_address);
        Ok(PyLightHandler::new(handler))
    }

    pub async fn l530(&self, ip_address: String) -> PyResult<PyColorLightHandler> {
        let handler: ColorLightHandler =
            call_handler_constructor!(self, tapo::ApiClient::l530, ip_address);
        Ok(PyColorLightHandler::new(handler))
    }

    pub async fn l535(&self, ip_address: String) -> PyResult<PyColorLightHandler> {
        let handler: ColorLightHandler =
            call_handler_constructor!(self, tapo::ApiClient::l535, ip_address);
        Ok(PyColorLightHandler::new(handler))
    }

    pub async fn l610(&self, ip_address: String) -> PyResult<PyLightHandler> {
        let handler: LightHandler =
            call_handler_constructor!(self, tapo::ApiClient::l610, ip_address);
        Ok(PyLightHandler::new(handler))
    }

    pub async fn l630(&self, ip_address: String) -> PyResult<PyColorLightHandler> {
        let handler: ColorLightHandler =
            call_handler_constructor!(self, tapo::ApiClient::l630, ip_address);
        Ok(PyColorLightHandler::new(handler))
    }

    pub async fn l900(&self, ip_address: String) -> PyResult<PyRgbLightStripHandler> {
        let handler: RgbLightStripHandler =
            call_handler_constructor!(self, tapo::ApiClient::l900, ip_address);
        Ok(PyRgbLightStripHandler::new(handler))
    }

    pub async fn l920(&self, ip_address: String) -> PyResult<PyRgbicLightStripHandler> {
        let handler: RgbicLightStripHandler =
            call_handler_constructor!(self, tapo::ApiClient::l920, ip_address);
        Ok(PyRgbicLightStripHandler::new(handler))
    }

    pub async fn l930(&self, ip_address: String) -> PyResult<PyRgbicLightStripHandler> {
        let handler: RgbicLightStripHandler =
            call_handler_constructor!(self, tapo::ApiClient::l930, ip_address);
        Ok(PyRgbicLightStripHandler::new(handler))
    }

    pub async fn p100(&self, ip_address: String) -> PyResult<PyPlugHandler> {
        let handler: PlugHandler =
            call_handler_constructor!(self, tapo::ApiClient::p100, ip_address);
        Ok(PyPlugHandler::new(handler))
    }

    pub async fn p105(&self, ip_address: String) -> PyResult<PyPlugHandler> {
        let handler: PlugHandler =
            call_handler_constructor!(self, tapo::ApiClient::p105, ip_address);
        Ok(PyPlugHandler::new(handler))
    }

    pub async fn p110(&self, ip_address: String) -> PyResult<PyPlugEnergyMonitoringHandler> {
        let handler: PlugEnergyMonitoringHandler =
            call_handler_constructor!(self, tapo::ApiClient::p110, ip_address);
        Ok(PyPlugEnergyMonitoringHandler::new(handler))
    }

    pub async fn p115(&self, ip_address: String) -> PyResult<PyPlugEnergyMonitoringHandler> {
        let handler: PlugEnergyMonitoringHandler =
            call_handler_constructor!(self, tapo::ApiClient::p115, ip_address);
        Ok(PyPlugEnergyMonitoringHandler::new(handler))
    }

    pub async fn p300(&self, ip_address: String) -> PyResult<PyPowerStripHandler> {
        let handler: PowerStripHandler =
            call_handler_constructor!(self, tapo::ApiClient::p300, ip_address);
        Ok(PyPowerStripHandler::new(handler))
    }

    pub async fn p304(&self, ip_address: String) -> PyResult<PyPowerStripHandler> {
        let handler: PowerStripHandler =
            call_handler_constructor!(self, tapo::ApiClient::p304, ip_address);
        Ok(PyPowerStripHandler::new(handler))
    }

    pub async fn h100(&self, ip_address: String) -> PyResult<PyHubHandler> {
        let handler: HubHandler =
            call_handler_constructor!(self, tapo::ApiClient::h100, ip_address);
        Ok(PyHubHandler::new(handler))
    }
}
