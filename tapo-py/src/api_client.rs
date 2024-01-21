use pyo3::prelude::*;
use tapo::ApiClient;

use crate::errors::ErrorWrapper;
use crate::handlers::{
    PyColorLightHandler, PyGenericDeviceHandler, PyLightHandler, PyPlugEnergyMonitoringHandler,
    PyPlugHandler,
};

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

    pub fn l510<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.l510(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyLightHandler::new(handler))
        })
    }

    pub fn l520<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.l520(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyLightHandler::new(handler))
        })
    }

    pub fn l530<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.l530(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyColorLightHandler::new(handler))
        })
    }

    pub fn l610<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.l610(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyLightHandler::new(handler))
        })
    }

    pub fn l630<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.l630(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyColorLightHandler::new(handler))
        })
    }

    pub fn l900<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.l900(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyColorLightHandler::new(handler))
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
