mod api_client;
mod errors;
mod handlers;
mod runtime;

use pyo3::prelude::*;

use api_client::PyApiClient;
use handlers::{
    PyColorLightHandler, PyColorLightSetDeviceInfoParams, PyEnergyDataInterval,
    PyGenericDeviceHandler, PyHubHandler, PyLightHandler, PyPlugEnergyMonitoringHandler,
    PyPlugHandler,
};
use tapo::requests::Color;
use tapo::responses::{
    ColorLightState, CurrentPowerResult, DefaultBrightnessState, DefaultColorLightState,
    DefaultLightState, DefaultPlugState, DefaultPowerType, DefaultStateType,
    DeviceInfoColorLightResult, DeviceInfoGenericResult, DeviceInfoHubResult,
    DeviceInfoLightResult, DeviceInfoPlugEnergyMonitoringResult, DeviceInfoPlugResult,
    DeviceUsageEnergyMonitoringResult, DeviceUsageResult, EnergyDataResult, EnergyUsageResult,
    KE100Result, OvercurrentStatus, OverheatStatus, PlugState, PowerProtectionStatus, S200BResult,
    Status, T100Result, T110Result, T300Result, T31XResult, TemperatureUnit, TemperatureUnitKE100,
    UsageByPeriodResult, WaterLeakStatus,
};

#[pymodule]
#[pyo3(name = "tapo")]
fn tapo_py(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyApiClient>()?;
    module.add_class::<PyColorLightHandler>()?;
    module.add_class::<PyGenericDeviceHandler>()?;
    module.add_class::<PyHubHandler>()?;
    module.add_class::<PyLightHandler>()?;
    module.add_class::<PyPlugEnergyMonitoringHandler>()?;
    module.add_class::<PyPlugHandler>()?;

    let requests = PyModule::new_bound(py, "tapo.requests")?;
    let responses = PyModule::new_bound(py, "tapo.responses")?;

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
    responses.add_class::<OvercurrentStatus>()?;
    responses.add_class::<OverheatStatus>()?;
    responses.add_class::<PowerProtectionStatus>()?;
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
    responses.add_class::<DefaultPlugState>()?;
    responses.add_class::<DeviceInfoPlugEnergyMonitoringResult>()?;
    responses.add_class::<DeviceInfoPlugResult>()?;
    responses.add_class::<PlugState>()?;

    module.add_submodule(&requests)?;
    module.add_submodule(&responses)?;

    let sys = py.import_bound("sys")?;
    let modules = sys.getattr("modules")?;
    modules.set_item("tapo.requests", requests)?;
    modules.set_item("tapo.responses", responses)?;

    Ok(())
}
