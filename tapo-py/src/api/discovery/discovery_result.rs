use pyo3::prelude::*;
use tapo::responses::{
    DeviceInfoColorLightResult, DeviceInfoGenericResult, DeviceInfoHubResult,
    DeviceInfoLightResult, DeviceInfoPlugEnergyMonitoringResult, DeviceInfoPlugResult,
    DeviceInfoPowerStripResult, DeviceInfoRgbLightStripResult, DeviceInfoRgbicLightStripResult,
};
use tapo::{DiscoveryResult, Error};

use crate::api::{
    PyColorLightHandler, PyGenericDeviceHandler, PyHubHandler, PyLightHandler,
    PyPlugEnergyMonitoringHandler, PyPlugHandler, PyPowerStripEnergyMonitoringHandler,
    PyPowerStripHandler, PyRgbLightStripHandler, PyRgbicLightStripHandler,
};
use crate::errors::ErrorWrapper;

#[pyclass(name = "DiscoveryResult")]
#[allow(clippy::large_enum_variant)]
pub enum PyDiscoveryResult {
    GenericDevice {
        device_info: DeviceInfoGenericResult,
        handler: PyGenericDeviceHandler,
    },
    Light {
        device_info: DeviceInfoLightResult,
        handler: PyLightHandler,
    },
    ColorLight {
        device_info: DeviceInfoColorLightResult,
        handler: PyColorLightHandler,
    },
    RgbLightStrip {
        device_info: DeviceInfoRgbLightStripResult,
        handler: PyRgbLightStripHandler,
    },
    RgbicLightStrip {
        device_info: DeviceInfoRgbicLightStripResult,
        handler: PyRgbicLightStripHandler,
    },
    Plug {
        device_info: DeviceInfoPlugResult,
        handler: PyPlugHandler,
    },
    PlugEnergyMonitoring {
        device_info: DeviceInfoPlugEnergyMonitoringResult,
        handler: PyPlugEnergyMonitoringHandler,
    },
    PowerStrip {
        device_info: DeviceInfoPowerStripResult,
        handler: PyPowerStripHandler,
    },
    PowerStripEnergyMonitoring {
        device_info: DeviceInfoPowerStripResult,
        handler: PyPowerStripEnergyMonitoringHandler,
    },
    Hub {
        device_info: DeviceInfoHubResult,
        handler: PyHubHandler,
    },
}

#[pyclass(name = "MaybeDiscoveryResult")]
pub struct PyMaybeDiscoveryResult {
    result: Option<PyDiscoveryResult>,
    exception: Option<ErrorWrapper>,
}

#[pymethods]
impl PyMaybeDiscoveryResult {
    pub fn get(mut slf: PyRefMut<'_, Self>) -> PyResult<PyDiscoveryResult> {
        if let Some(result) = slf.result.take() {
            Ok(result)
        } else if let Some(exception) = slf.exception.take() {
            Err(exception.into())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "No result or exception available. `get` can only be called once.",
            ))
        }
    }
}

pub fn convert_result_to_maybe_py(
    result: Result<DiscoveryResult, Error>,
) -> PyResult<PyMaybeDiscoveryResult> {
    match result {
        Ok(result) => Ok(PyMaybeDiscoveryResult {
            result: Some(convert_result_to_py(result)),
            exception: None,
        }),
        Err(e) => Ok(PyMaybeDiscoveryResult {
            result: None,
            exception: Some(ErrorWrapper::from(e)),
        }),
    }
}

fn convert_result_to_py(result: DiscoveryResult) -> PyDiscoveryResult {
    match result {
        DiscoveryResult::GenericDevice {
            device_info,
            handler,
        } => PyDiscoveryResult::GenericDevice {
            device_info: *device_info,
            handler: PyGenericDeviceHandler::new(handler),
        },
        DiscoveryResult::Light {
            device_info,
            handler,
        } => PyDiscoveryResult::Light {
            device_info: *device_info,
            handler: PyLightHandler::new(handler),
        },
        DiscoveryResult::ColorLight {
            device_info,
            handler,
        } => PyDiscoveryResult::ColorLight {
            device_info: *device_info,
            handler: PyColorLightHandler::new(handler),
        },
        DiscoveryResult::RgbLightStrip {
            device_info,
            handler,
        } => PyDiscoveryResult::RgbLightStrip {
            device_info: *device_info,
            handler: PyRgbLightStripHandler::new(handler),
        },
        DiscoveryResult::RgbicLightStrip {
            device_info,
            handler,
        } => PyDiscoveryResult::RgbicLightStrip {
            device_info: *device_info,
            handler: PyRgbicLightStripHandler::new(handler),
        },
        DiscoveryResult::Plug {
            device_info,
            handler,
        } => PyDiscoveryResult::Plug {
            device_info: *device_info,
            handler: PyPlugHandler::new(handler),
        },
        DiscoveryResult::PlugEnergyMonitoring {
            device_info,
            handler,
        } => PyDiscoveryResult::PlugEnergyMonitoring {
            device_info: *device_info,
            handler: PyPlugEnergyMonitoringHandler::new(handler),
        },
        DiscoveryResult::PowerStrip {
            device_info,
            handler,
        } => PyDiscoveryResult::PowerStrip {
            device_info: *device_info,
            handler: PyPowerStripHandler::new(handler),
        },
        DiscoveryResult::PowerStripEnergyMonitoring {
            device_info,
            handler,
        } => PyDiscoveryResult::PowerStripEnergyMonitoring {
            device_info: *device_info,
            handler: PyPowerStripEnergyMonitoringHandler::new(handler),
        },
        DiscoveryResult::Hub {
            device_info,
            handler,
        } => PyDiscoveryResult::Hub {
            device_info: *device_info,
            handler: PyHubHandler::new(handler),
        },
    }
}
