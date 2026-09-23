use pyo3::prelude::*;
use serde::Serialize;
use tapo::responses::{AiCameraSupport, BackupWifi, GeneralDeviceHubResult};

/// General device (standalone Wi-Fi camera) paired to a camera hub.
///
/// Wraps the Rust result so that `backup_wifi` can be exposed as
/// [`PyBackupWifi`]. `to_dict` serializes the wrapped value, so
/// `backup_wifi` keeps its raw wire value there.
#[derive(Debug, Clone, Serialize)]
#[pyclass(name = "GeneralDeviceHubResult", from_py_object)]
#[serde(transparent)]
pub struct PyGeneralDeviceHubResult(GeneralDeviceHubResult);

impl From<GeneralDeviceHubResult> for PyGeneralDeviceHubResult {
    fn from(result: GeneralDeviceHubResult) -> Self {
        Self(result)
    }
}

#[pymethods]
impl PyGeneralDeviceHubResult {
    /// The AI detection types the camera runs itself.
    #[getter]
    fn ai_camera_support(&self) -> AiCameraSupport {
        self.0.ai_camera_support
    }

    #[getter]
    fn alias(&self) -> String {
        self.0.alias.clone()
    }

    /// The backup Wi-Fi network.
    #[getter]
    fn backup_wifi(&self) -> PyBackupWifi {
        PyBackupWifi::from(&self.0.backup_wifi)
    }

    #[getter]
    fn category(&self) -> String {
        self.0.category.clone()
    }

    #[getter]
    fn device_id(&self) -> String {
        self.0.device_id.clone()
    }

    #[getter]
    fn device_model(&self) -> String {
        self.0.device_model.clone()
    }

    #[getter]
    fn device_type(&self) -> String {
        self.0.device_type.clone()
    }

    /// Whether the hub stores this camera's footage.
    #[getter]
    fn hub_storage_enabled(&self) -> bool {
        self.0.hub_storage_enabled
    }

    #[getter]
    fn mac(&self) -> String {
        self.0.mac.clone()
    }

    #[getter]
    fn network_mode(&self) -> String {
        self.0.network_mode.clone()
    }

    #[getter]
    fn parent_device_id(&self) -> String {
        self.0.parent_device_id.clone()
    }

    /// Whether 24h continuous recording is enabled for this camera.
    #[getter]
    fn plan_24h_record(&self) -> bool {
        self.0.plan_24h_record
    }

    #[getter]
    fn wifi_backup_enabled(&self) -> bool {
        self.0.wifi_backup_enabled
    }
}

tapo::impl_to_dict!(PyGeneralDeviceHubResult);

/// The backup Wi-Fi network of a camera paired to a camera hub. PyO3 cannot
/// expose a Rust enum that mixes unit variants with a data-carrying one, so
/// this enum mirrors [`BackupWifi`] with struct variants. `None` becomes
/// `Disabled` because `BackupWifi.None` is not valid Python.
#[derive(Debug, Clone, PartialEq, Eq)]
#[pyclass(name = "BackupWifi", from_py_object, eq, frozen)]
#[allow(missing_docs)]
pub enum PyBackupWifi {
    Auto {},
    Disabled {},
    Ssid { ssid: String },
}

impl From<&BackupWifi> for PyBackupWifi {
    fn from(value: &BackupWifi) -> Self {
        match value {
            BackupWifi::Auto => Self::Auto {},
            BackupWifi::None => Self::Disabled {},
            BackupWifi::Ssid(ssid) => Self::Ssid { ssid: ssid.clone() },
        }
    }
}
