mod api_client;
mod errors;
mod handlers;

use pyo3::prelude::*;

use api_client::PyApiClient;
use handlers::{PyColorLightSetDeviceInfoParams, PyEnergyDataInterval};
use tapo::requests::Color;
use tapo::responses::{
    ColorLightState, CurrentPowerResult, DefaultBrightnessState, DefaultColorLightState,
    DefaultLightState, DefaultPlugState, DefaultPowerType, DefaultStateType,
    DeviceInfoColorLightResult, DeviceInfoGenericResult, DeviceInfoHubResult,
    DeviceInfoLightResult, DeviceInfoPlugResult, DeviceUsageEnergyMonitoringResult,
    DeviceUsageResult, EnergyDataResult, EnergyUsageResult, KE100Result, PlugState, S200BResult,
    Status, T100Result, T110Result, T300Result, T31XResult, TemperatureUnit, TemperatureUnitKE100,
    UsageByPeriodResult, WaterLeakStatus,
};

#[pymodule]
#[pyo3(name = "tapo")]
fn tapo_py(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyApiClient>()?;

    let requests = PyModule::new(py, "tapo.requests")?;
    let responses = PyModule::new(py, "tapo.responses")?;

    // requests
    requests.add_class::<PyEnergyDataInterval>()?;
    requests.add_class::<Color>()?;
    requests.add_class::<PyColorLightSetDeviceInfoParams>()?;

    // responses
    responses.add_class::<CurrentPowerResult>()?;
    responses.add_class::<DefaultBrightnessState>()?;
    responses.add_class::<DefaultPowerType>()?;
    responses.add_class::<DefaultStateType>()?;
    responses.add_class::<DeviceUsageEnergyMonitoringResult>()?;
    responses.add_class::<DeviceUsageResult>()?;
    responses.add_class::<EnergyDataResult>()?;
    responses.add_class::<EnergyUsageResult>()?;
    responses.add_class::<UsageByPeriodResult>()?;

    // responses: device info: color light
    responses.add_class::<DeviceInfoColorLightResult>()?;
    responses.add_class::<DefaultColorLightState>()?;
    responses.add_class::<ColorLightState>()?;

    // responses: device info: generic
    responses.add_class::<DeviceInfoGenericResult>()?;

    // responses: hub
    responses.add_class::<DeviceInfoHubResult>()?;
    responses.add_class::<KE100Result>()?;
    responses.add_class::<S200BResult>()?;
    responses.add_class::<T100Result>()?;
    responses.add_class::<T110Result>()?;
    responses.add_class::<T300Result>()?;
    responses.add_class::<T31XResult>()?;

    // responses: hub devices
    responses.add_class::<Status>()?;
    responses.add_class::<TemperatureUnit>()?;
    responses.add_class::<TemperatureUnitKE100>()?;
    responses.add_class::<WaterLeakStatus>()?;

    // responses: light
    responses.add_class::<DeviceInfoLightResult>()?;
    responses.add_class::<DefaultLightState>()?;

    // responses: plug
    responses.add_class::<DeviceInfoPlugResult>()?;
    responses.add_class::<DefaultPlugState>()?;
    responses.add_class::<PlugState>()?;

    let sys = py.import("sys")?;
    let modules = sys.getattr("modules")?;
    modules.set_item("tapo.requests", requests)?;
    modules.set_item("tapo.responses", responses)?;

    module.add_submodule(requests)?;
    module.add_submodule(responses)?;

    Ok(())
}
