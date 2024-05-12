use pyo3::prelude::*;
use std::time::Duration;
use tapo::ApiClient;

use crate::errors::ErrorWrapper;
use crate::handlers::{
    PyColorLightHandler, PyGenericDeviceHandler, PyHubHandler, PyLightHandler,
    PyPlugEnergyMonitoringHandler, PyPlugHandler,
};

#[pyclass(name = "ApiClient")]
pub struct PyApiClient {
    client: ApiClient,
}

#[pymethods]
impl PyApiClient {
    #[new]
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
        let client = self.client.clone();
        let handler = client
            .generic_device(ip_address)
            .await
            .map_err(ErrorWrapper)?;
        Ok(PyGenericDeviceHandler::new(handler))
    }

    pub async fn l510(&self, ip_address: String) -> PyResult<PyLightHandler> {
        let client = self.client.clone();
        let handler = client.l510(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyLightHandler::new(handler))
    }

    pub async fn l520(&self, ip_address: String) -> PyResult<PyLightHandler> {
        let client = self.client.clone();
        let handler = client.l520(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyLightHandler::new(handler))
    }

    pub async fn l530(&self, ip_address: String) -> PyResult<PyColorLightHandler> {
        let client = self.client.clone();
        let handler = client.l530(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyColorLightHandler::new(handler))
    }

    pub async fn l610(&self, ip_address: String) -> PyResult<PyLightHandler> {
        let client = self.client.clone();
        let handler = client.l610(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyLightHandler::new(handler))
    }

    pub async fn l630(&self, ip_address: String) -> PyResult<PyColorLightHandler> {
        let client = self.client.clone();
        let handler = client.l630(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyColorLightHandler::new(handler))
    }

    pub async fn p100(&self, ip_address: String) -> PyResult<PyPlugHandler> {
        let client = self.client.clone();
        let handler = client.p100(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyPlugHandler::new(handler))
    }

    pub async fn p105(&self, ip_address: String) -> PyResult<PyPlugHandler> {
        let client = self.client.clone();
        let handler = client.p105(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyPlugHandler::new(handler))
    }

    pub async fn p110(&self, ip_address: String) -> PyResult<PyPlugEnergyMonitoringHandler> {
        let client = self.client.clone();
        let handler = client.p110(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyPlugEnergyMonitoringHandler::new(handler))
    }

    pub async fn p115(&self, ip_address: String) -> PyResult<PyPlugEnergyMonitoringHandler> {
        let client = self.client.clone();
        let handler = client.p115(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyPlugEnergyMonitoringHandler::new(handler))
    }

    pub async fn h100(&self, ip_address: String) -> PyResult<PyHubHandler> {
        let client = self.client.clone();
        let handler = client.h100(ip_address).await.map_err(ErrorWrapper)?;
        Ok(PyHubHandler::new(handler))
    }
}
