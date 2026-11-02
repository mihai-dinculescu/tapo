from datetime import date
from typing import Optional

from tapo.to_dict_ext import ToDictExt

class LatestFirmwareResult(ToDictExt):
    """Contains the newest firmware that the device can update to."""

    update_available: bool
    """`True` when a firmware update is available."""
    fw_ver: str
    """Version of the newest firmware. When no update is available, this is the
    version that the device is running."""
    release_date: Optional[date]
    """Release date of the update. `None` when no update is available."""
    release_notes: Optional[str]
    """Release notes of the update. `None` when no update is available."""
