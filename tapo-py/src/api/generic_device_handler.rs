#[allow(unused_imports)]
use pyo3::prelude::*;
use tapo::GenericDeviceHandler;
use tapo::responses::DeviceInfoGenericResult;

py_handler! {
    PyGenericDeviceHandler(GenericDeviceHandler, DeviceInfoGenericResult),
    py_name = "GenericDeviceHandler",
    on_off,
}
