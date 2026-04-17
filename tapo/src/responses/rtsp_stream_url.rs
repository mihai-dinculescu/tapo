use serde::{Deserialize, Serialize};

/// RTSP stream URLs for the camera.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct RtspStreamUrl {
    /// High-definition stream URL (stream1).
    pub hd: String,
    /// Standard-definition stream URL (stream2).
    pub sd: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(RtspStreamUrl);
