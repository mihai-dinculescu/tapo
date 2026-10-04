use std::net::IpAddr;

use serde_json::Value;

use crate::api::protocol::{AuthProtocol, DeviceFamily, TpapInfo};
use crate::error::Error;

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

    /// The protocol the device announces.
    ///
    /// # Errors
    ///
    /// Returns an error if the device announces a protocol that is not supported,
    /// or TPAP in a way that cannot be read.
    pub(crate) fn auth_protocol(&self) -> Result<AuthProtocol, Error> {
        let result = self.message.get("result");
        let scheme = result.and_then(|r| r.get("mgt_encrypt_schm"));
        let tpap = result
            .and_then(|r| r.get("tpap"))
            .filter(|tpap| tpap.is_object());

        if scheme.and_then(|s| s["is_support_https"].as_bool()) == Some(true) {
            // A camera that takes TPAP announces a `tpap` object, and one
            // that does not take it announces none.
            return match tpap {
                Some(tpap) => Ok(AuthProtocol::TpapAnnounced(TpapInfo::from_announcement(
                    tpap,
                )?)),
                None => Ok(AuthProtocol::AesSsl),
            };
        }

        match (scheme.and_then(|s| s["encrypt_type"].as_str()), tpap) {
            (Some("KLAP"), _) => Ok(AuthProtocol::Klap),
            (Some("AES"), _) => Err(Error::unsupported_aes_protocol()),
            // A device in TPAP mode announces a `tpap` object as well, which
            // says how it wants to be logged in to.
            (Some("TPAP"), Some(tpap)) => Ok(AuthProtocol::TpapAnnounced(
                TpapInfo::from_announcement(tpap)?,
            )),
            _ => Ok(AuthProtocol::Unknown),
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
        encrypt_type: Option<&str>,
    ) -> DiscoveryRawResult {
        let mut mgt_encrypt_schm = json!({ "is_support_https": is_support_https });
        if let Some(encrypt_type) = encrypt_type {
            mgt_encrypt_schm["encrypt_type"] = json!(encrypt_type);
        }

        DiscoveryRawResult {
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            message: json!({
                "result": {
                    "device_type": device_type,
                    "device_model": device_model,
                    "mgt_encrypt_schm": mgt_encrypt_schm,
                },
                "error_code": 0,
            }),
        }
    }

    #[test]
    fn klap_hint_uses_klap_protocol() {
        let result = raw_result("SMART.TAPOPLUG", "P110", false, Some("KLAP"));

        assert_eq!(result.device_family(), DeviceFamily::Smart);
        assert_eq!(result.auth_protocol().unwrap(), AuthProtocol::Klap);
    }

    #[test]
    fn aes_hint_is_an_error() {
        let result = raw_result("SMART.TAPOPLUG", "P110", false, Some("AES"));

        assert!(matches!(
            result.auth_protocol().err(),
            Some(Error::UnsupportedProtocol { protocol, .. }) if protocol == "AES"
        ));
    }

    #[test]
    fn tpap_hint_uses_the_announced_tpap_protocol() {
        // What an L930 announces in TPAP mode.
        let mut result = raw_result("SMART.TAPOBULB", "L930", false, Some("TPAP"));
        result.message["result"]["tpap"] =
            json!({ "tls": 0, "dac": 0, "noc": 0, "pake": [2], "port": 80 });

        assert_eq!(result.device_family(), DeviceFamily::Smart);
        assert!(matches!(
            result.auth_protocol().unwrap(),
            AuthProtocol::TpapAnnounced(_)
        ));
    }

    #[test]
    fn tpap_hint_without_a_tpap_object_is_unknown() {
        let result = raw_result("SMART.TAPOBULB", "L930", false, Some("TPAP"));

        assert_eq!(result.auth_protocol().unwrap(), AuthProtocol::Unknown);
    }

    #[test]
    fn missing_hint_is_unknown() {
        let result = raw_result("SMART.TAPOHUB", "H100", false, None);

        assert_eq!(result.auth_protocol().unwrap(), AuthProtocol::Unknown);
    }

    #[test]
    fn camera_hub_uses_smart_cam_family() {
        let result = raw_result("SMART.TAPOHUB", "H200", true, None);

        assert!(result.is_camera_hub());
        assert_eq!(result.device_family(), DeviceFamily::SmartCam);
        assert_eq!(result.auth_protocol().unwrap(), AuthProtocol::AesSsl);
    }

    #[test]
    fn hub_uses_smart_family() {
        let result = raw_result("SMART.TAPOHUB", "H100", false, None);

        assert!(!result.is_camera_hub());
        assert_eq!(result.device_family(), DeviceFamily::Smart);
    }

    #[test]
    fn camera_uses_smart_cam_family() {
        let result = raw_result("SMART.IPCAMERA", "C220", true, None);

        assert!(!result.is_camera_hub());
        assert_eq!(result.device_family(), DeviceFamily::SmartCam);
        assert_eq!(result.auth_protocol().unwrap(), AuthProtocol::AesSsl);
    }

    #[test]
    fn camera_with_a_tpap_object_uses_the_announced_tpap_protocol() {
        // What a C220 announces.
        let mut result = raw_result("SMART.IPCAMERA", "C220", true, None);
        result.message["result"]["tpap"] = json!({
            "noc": 1,
            "pake": [2],
            "port": 443,
            "tls": 1,
        });

        assert_eq!(result.device_family(), DeviceFamily::SmartCam);
        assert!(matches!(
            result.auth_protocol().unwrap(),
            AuthProtocol::TpapAnnounced(_)
        ));
    }

    #[test]
    fn camera_with_an_unreadable_tpap_object_is_an_error() {
        let mut result = raw_result("SMART.IPCAMERA", "C220", true, None);
        result.message["result"]["tpap"] = json!({ "pake": "2" });

        assert!(matches!(
            result.auth_protocol().err(),
            Some(Error::UnsupportedProtocol { protocol, .. }) if protocol == "TPAP"
        ));
    }
}
