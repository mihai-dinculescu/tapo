use pyo3::prelude::*;
use tapo::responses::{
    DeviceInfoBasicResult, DeviceInfoColorLightResult, DeviceInfoHubResult, DeviceInfoLightResult,
    DeviceInfoPlugEnergyMonitoringResult, DeviceInfoPlugResult, DeviceInfoPowerStripResult,
    DeviceInfoRgbLightStripResult, DeviceInfoRgbicLightStripResult,
};
use tapo::{DeviceType, DiscoveryError, DiscoveryResult};

use crate::api::{
    PyColorLightHandler, PyHubHandler, PyLightHandler, PyPlugEnergyMonitoringHandler,
    PyPlugHandler, PyPowerStripEnergyMonitoringHandler, PyPowerStripHandler,
    PyRgbLightStripHandler, PyRgbicLightStripHandler,
};
#[pyclass(name = "DiscoveryResult")]
#[allow(clippy::large_enum_variant)]
pub enum PyDiscoveryResult {
    Other {
        device_info: DeviceInfoBasicResult,
        ip: String,
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

#[pymethods]
impl PyDiscoveryResult {
    #[getter]
    pub fn device_type(&self) -> DeviceType {
        match self {
            PyDiscoveryResult::Light { .. } => DeviceType::Light,
            PyDiscoveryResult::ColorLight { .. } => DeviceType::ColorLight,
            PyDiscoveryResult::RgbLightStrip { .. } => DeviceType::RgbLightStrip,
            PyDiscoveryResult::RgbicLightStrip { .. } => DeviceType::RgbicLightStrip,
            PyDiscoveryResult::Plug { .. } => DeviceType::Plug,
            PyDiscoveryResult::PlugEnergyMonitoring { .. } => DeviceType::PlugEnergyMonitoring,
            PyDiscoveryResult::PowerStrip { .. } => DeviceType::PowerStrip,
            PyDiscoveryResult::PowerStripEnergyMonitoring { .. } => {
                DeviceType::PowerStripEnergyMonitoring
            }
            PyDiscoveryResult::Hub { .. } => DeviceType::Hub,
            PyDiscoveryResult::Other { .. } => DeviceType::Other,
        }
    }

    #[getter]
    pub fn model(&self) -> &str {
        match self {
            PyDiscoveryResult::Light { device_info, .. } => &device_info.model,
            PyDiscoveryResult::ColorLight { device_info, .. } => &device_info.model,
            PyDiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.model,
            PyDiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.model,
            PyDiscoveryResult::Plug { device_info, .. } => &device_info.model,
            PyDiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.model,
            PyDiscoveryResult::PowerStrip { device_info, .. } => &device_info.model,
            PyDiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => &device_info.model,
            PyDiscoveryResult::Hub { device_info, .. } => &device_info.model,
            PyDiscoveryResult::Other { device_info, .. } => &device_info.model,
        }
    }

    #[getter]
    pub fn ip(&self) -> &str {
        match self {
            PyDiscoveryResult::Light { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::ColorLight { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::Plug { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::PowerStrip { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::Hub { device_info, .. } => &device_info.ip,
            PyDiscoveryResult::Other { ip, .. } => ip,
        }
    }

    #[getter]
    pub fn device_id(&self) -> &str {
        match self {
            PyDiscoveryResult::Light { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::ColorLight { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::Plug { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::PowerStrip { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => {
                &device_info.device_id
            }
            PyDiscoveryResult::Hub { device_info, .. } => &device_info.device_id,
            PyDiscoveryResult::Other { device_info, .. } => &device_info.device_id,
        }
    }

    #[getter]
    pub fn nickname(&self) -> &str {
        match self {
            PyDiscoveryResult::Light { device_info, .. } => &device_info.nickname,
            PyDiscoveryResult::ColorLight { device_info, .. } => &device_info.nickname,
            PyDiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.nickname,
            PyDiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.nickname,
            PyDiscoveryResult::Plug { device_info, .. } => &device_info.nickname,
            PyDiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.nickname,
            PyDiscoveryResult::PowerStrip { .. } => DeviceType::PowerStrip.as_str(),
            PyDiscoveryResult::PowerStripEnergyMonitoring { .. } => {
                DeviceType::PowerStripEnergyMonitoring.as_str()
            }
            PyDiscoveryResult::Hub { device_info, .. } => &device_info.nickname,
            PyDiscoveryResult::Other { device_info, .. } => device_info
                .nickname
                .as_deref()
                .unwrap_or(DeviceType::Other.as_str()),
        }
    }
}

#[pyclass(name = "MaybeDiscoveryResult")]
pub struct PyMaybeDiscoveryResult {
    result: Option<PyDiscoveryResult>,
    exception: Option<DiscoveryError>,
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
    result: Result<DiscoveryResult, DiscoveryError>,
) -> PyResult<PyMaybeDiscoveryResult> {
    match result {
        Ok(result) => Ok(PyMaybeDiscoveryResult {
            result: Some(convert_result_to_py(result)),
            exception: None,
        }),
        Err(e) => Ok(PyMaybeDiscoveryResult {
            result: None,
            exception: Some(e),
        }),
    }
}

fn convert_result_to_py(result: DiscoveryResult) -> PyDiscoveryResult {
    match result {
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
        DiscoveryResult::Other { device_info, ip } => PyDiscoveryResult::Other {
            device_info: *device_info,
            ip,
        },
    }
}
