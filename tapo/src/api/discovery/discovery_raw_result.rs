use std::net::IpAddr;

use serde_json::Value;

use crate::api::protocol::{AuthProtocol, DeviceFamily};

/// A raw discovery response containing the device's IP address and the
/// JSON message received through the UDP discovery stream.
#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(not(feature = "debug"), allow(unreachable_pub))]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, name = "DiscoveryRawResult")
)]
pub struct DiscoveryRawResult {
    /// The IP address of the responding device.
    pub ip: IpAddr,
    /// The JSON message payload from the discovery response.
    pub message: Value,
}

#[cfg(feature = "python")]
#[pyo3::prelude::pymethods]
impl DiscoveryRawResult {
    #[getter]
    fn get_ip(&self) -> String {
        self.ip.to_string()
    }

    #[getter]
    fn get_message(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        crate::python::serde_object_to_py_dict(py, &self.message)
    }
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DiscoveryRawResult);

impl DiscoveryRawResult {
    pub(crate) fn device_family(&self) -> DeviceFamily {
        match self
            .message
            .get("result")
            .and_then(|r| r.get("device_type"))
            .and_then(|v| v.as_str())
        {
            Some("SMART.IPCAMERA") => DeviceFamily::SmartCam,
            _ => DeviceFamily::Smart,
        }
    }

    pub(crate) fn auth_protocol(&self) -> AuthProtocol {
        let scheme = self
            .message
            .get("result")
            .and_then(|r| r.get("mgt_encrypt_schm"));

        if scheme.and_then(|s| s["is_support_https"].as_bool()) == Some(true) {
            return AuthProtocol::AesSsl;
        }

        match scheme.and_then(|s| s["encrypt_type"].as_str()) {
            Some("KLAP") => AuthProtocol::Klap,
            Some("AES") => AuthProtocol::Aes,
            _ => AuthProtocol::Unknown,
        }
    }
}
