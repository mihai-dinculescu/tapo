use pyo3::prelude::*;
use tapo::PlugHandler;
use tapo::responses::{DeviceInfoPlugResult, DeviceUsageResult};

py_handler! {
    PyPlugHandler(PlugHandler, DeviceInfoPlugResult),
    py_name = "PlugHandler",
    on_off,
    device_management,
    device_usage = DeviceUsageResult,
}
