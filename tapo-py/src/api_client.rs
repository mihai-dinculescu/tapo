use pyo3::prelude::*;
use tapo::ApiClient;

use crate::errors::ErrorWrapper;
use crate::handlers::{PyGenericDeviceHandler, PyPlugEnergyMonitoringHandler, PyPlugHandler};

#[pyclass(name = "ApiClient")]
pub struct PyApiClient {
    client: ApiClient,
}

#[pymethods]
impl PyApiClient {
    #[new]
    pub fn new(tapo_username: String, tapo_password: String) -> Result<Self, ErrorWrapper> {
        let client = ApiClient::new(tapo_username, tapo_password)?;
        Ok(Self { client })
    }

    pub fn generic_device<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client
                .generic_device(ip_address)
                .await
                .map_err(ErrorWrapper)?;
            Ok(PyGenericDeviceHandler::new(handler))
        })
    }

    pub fn p100<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.p100(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyPlugHandler::new(handler))
        })
    }

    pub fn p105<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.p105(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyPlugHandler::new(handler))
        })
    }

    pub fn p110<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.p110(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyPlugEnergyMonitoringHandler::new(handler))
        })
    }

    pub fn p115<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.p115(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyPlugEnergyMonitoringHandler::new(handler))
        })
    }
}
