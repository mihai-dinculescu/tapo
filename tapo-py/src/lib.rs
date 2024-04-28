mod api_client;
mod errors;
mod handlers;

use pyo3::prelude::*;

use api_client::PyApiClient;
use handlers::PyEnergyDataInterval;
use tapo::requests::Color;
use tapo::responses::{
    KE100Result, S200BResult, Status, T100Result, T110Result, T300Result, T31XResult,
    TemperatureUnit, TemperatureUnitKE100, WaterLeakStatus,
};

#[pymodule]
#[pyo3(name = "tapo")]
fn tapo_py(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyApiClient>()?;

    module.add_class::<PyEnergyDataInterval>()?;
    module.add_class::<Color>()?;

    let responses = PyModule::new(py, "tapo.responses")?;

    responses.add_class::<Status>()?;
    responses.add_class::<TemperatureUnit>()?;
    responses.add_class::<TemperatureUnitKE100>()?;
    responses.add_class::<WaterLeakStatus>()?;

    responses.add_class::<KE100Result>()?;
    responses.add_class::<S200BResult>()?;
    responses.add_class::<T100Result>()?;
    responses.add_class::<T110Result>()?;
    responses.add_class::<T300Result>()?;
    responses.add_class::<T31XResult>()?;

    let sys = py.import("sys")?;
    let modules = sys.getattr("modules")?;
    modules.set_item("tapo.responses", responses)?;

    module.add_submodule(responses)?;

    Ok(())
}
