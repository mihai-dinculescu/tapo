use std::net::IpAddr;

use serde_json::Value;

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

impl From<&DiscoveryRawResult> for crate::api::protocol::AuthProtocol {
    fn from(raw: &DiscoveryRawResult) -> Self {
        use crate::api::protocol::AuthProtocol;

        let scheme = raw
            .message
            .get("result")
            .and_then(|r| r.get("mgt_encrypt_schm"));

        let encrypt_type = scheme.and_then(|s| s["encrypt_type"].as_str());
        let http_port = scheme.and_then(|s| s["http_port"].as_u64());

        match (encrypt_type, http_port) {
            (Some("KLAP"), Some(80)) => AuthProtocol::Klap,
            (Some("AES"), Some(80)) => AuthProtocol::Aes,
            _ => AuthProtocol::Unknown,
        }
    }
}
