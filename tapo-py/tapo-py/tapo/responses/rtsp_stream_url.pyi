from tapo.to_dict_ext import ToDictExt

class RtspStreamUrl(ToDictExt):
    """RTSP stream URLs for the camera."""

    hd: str
    """High-definition H.264 stream URL (stream1)."""

    sd: str
    """Standard-definition H.264 stream URL (stream2)."""

    mjpeg: str
    """Motion JPEG stream URL (stream8). Suitable for frame-by-frame capture
    without an H.264 decoder; each frame is a self-contained JPEG."""
