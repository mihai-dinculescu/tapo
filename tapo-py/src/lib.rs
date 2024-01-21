mod api_client;
mod errors;
mod handlers;

use pyo3::prelude::*;

use api_client::PyApiClient;
use handlers::PyEnergyDataInterval;
use tapo::requests::Color;

#[pymodule]
#[pyo3(name = "tapo")]
fn tapo_py(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyApiClient>()?;

    module.add_class::<PyEnergyDataInterval>()?;
    module.add_class::<Color>()?;

    Ok(())
}
