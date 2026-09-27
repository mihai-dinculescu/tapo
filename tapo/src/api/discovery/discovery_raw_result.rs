use std::net::IpAddr;

use serde_json::Value;

use crate::api::protocol::{AuthProtocol, DeviceFamily};

use super::DeviceType;

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
        // Camera hubs announce themselves as `SMART.TAPOHUB`, like the H100,
        // but speak the SmartCam protocol.
        if self.is_camera_hub() {
            return DeviceFamily::SmartCam;
        }

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

    pub(crate) fn is_camera_hub(&self) -> bool {
        self.message
            .get("result")
            .and_then(|r| r.get("device_model"))
            .and_then(|v| v.as_str())
            .map(DeviceType::from_model)
            == Some(DeviceType::CameraHub)
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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use serde_json::json;

    use super::*;

    fn raw_result(
        device_type: &str,
        device_model: &str,
        is_support_https: bool,
    ) -> DiscoveryRawResult {
        DiscoveryRawResult {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            message: json!({
                "result": {
                    "device_type": device_type,
                    "device_model": device_model,
                    "mgt_encrypt_schm": { "is_support_https": is_support_https },
                },
                "error_code": 0,
            }),
        }
    }

    #[test]
    fn camera_hub_uses_smart_cam_family() {
        let result = raw_result("SMART.TAPOHUB", "H200", true);

        assert!(result.is_camera_hub());
        assert_eq!(result.device_family(), DeviceFamily::SmartCam);
        assert_eq!(result.auth_protocol(), AuthProtocol::AesSsl);
    }

    #[test]
    fn hub_uses_smart_family() {
        let result = raw_result("SMART.TAPOHUB", "H100", false);

        assert!(!result.is_camera_hub());
        assert_eq!(result.device_family(), DeviceFamily::Smart);
    }

    #[test]
    fn camera_uses_smart_cam_family() {
        let result = raw_result("SMART.IPCAMERA", "C220", true);

        assert!(!result.is_camera_hub());
        assert_eq!(result.device_family(), DeviceFamily::SmartCam);
    }
}
