from typing import Optional

from tapo.to_dict_ext import ToDictExt

class RecordingDownloadResult(ToDictExt):
    """The result of downloading a recording stored on a camera hub with
    ``CameraHubHandler.download_recording``."""

    byte_count: int
    """The number of media bytes written, after decryption."""

    duration_s: Optional[float]
    """The playback time the media covers, in seconds, measured between the
    first and the last MPEG-TS clock reference. The H200 sends one about
    every 2 s, so media after the last one is not counted and this can
    fall short of what was written by up to that much. ``None`` when the
    hub sent a stream without a usable clock, in which case the length of
    the download is unknown."""
