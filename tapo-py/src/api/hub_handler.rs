use pyo3::prelude::*;
use tapo::HubHandler;
use tapo::responses::DeviceInfoHubResult;

py_handler! {
    PyHubHandler(HubHandler, DeviceInfoHubResult),
    py_name = "HubHandler",
    device_management,
}

py_hub_alarm_handlers!(PyHubHandler, HubHandler);

py_hub_child_handlers!(PyHubHandler, HubHandler, ChildDeviceHubResult);
