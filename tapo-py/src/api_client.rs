use pyo3::prelude::*;
use tapo::ApiClient;

use crate::errors::ErrorWrapper;
use crate::handlers::PyEnergyMonitoringPlugHandler;

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

    pub fn p110<'a>(&'a self, ip_address: String, py: Python<'a>) -> PyResult<&'a PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let handler = client.p110(ip_address).await.map_err(ErrorWrapper)?;
            Ok(PyEnergyMonitoringPlugHandler::new(handler))
        })
    }
}
