mod api_client;
mod errors;
mod handlers;

use pyo3::prelude::*;
use tapo::responses::PlugDeviceInfoResult;

use api_client::PyApiClient;
use handlers::PyEnergyMonitoringPlugHandler;

#[pymodule]
#[pyo3(name = "tapo")]
fn tapo_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyApiClient>()?;
    m.add_class::<PyEnergyMonitoringPlugHandler>()?;
    m.add_class::<PlugDeviceInfoResult>()?;
    Ok(())
}
