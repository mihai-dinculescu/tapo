from tapo.to_dict_ext import ToDictExt

class RtspStreamUrl(ToDictExt):
    """RTSP stream URLs for the camera."""

    hd: str
    """High-definition stream URL (stream1)."""

    sd: str
    """Standard-definition stream URL (stream2)."""
