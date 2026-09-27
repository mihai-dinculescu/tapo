use serde::{Deserialize, Serialize};

/// RTSP stream URLs for the camera.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct RtspStreamUrl {
    /// High-definition H.264 stream URL (stream1).
    pub hd: String,
    /// Standard-definition H.264 stream URL (stream2).
    pub sd: String,
    /// Motion JPEG stream URL (stream8). Suitable for frame-by-frame capture
    /// without an H.264 decoder; each frame is a self-contained JPEG.
    pub mjpeg: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(RtspStreamUrl);
