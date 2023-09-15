mod api_client;
mod errors;
mod handlers;

use pyo3::prelude::*;
use tapo::responses::DeviceInfoPlugResult;

use api_client::PyApiClient;
use handlers::PyPlugEnergyMonitoringHandler;

#[pymodule]
#[pyo3(name = "tapo")]
fn tapo_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyApiClient>()?;
    m.add_class::<PyPlugEnergyMonitoringHandler>()?;
    m.add_class::<DeviceInfoPlugResult>()?;
    Ok(())
}
