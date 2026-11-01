from typing import Optional
from tapo.to_dict_ext import ToDictExt

class LatestFirmwareResult(ToDictExt):
    """Contains the latest firmware that the device reports as available."""

    fw_ver: Optional[str]
    """Version string of the latest firmware."""
    need_to_upgrade: Optional[bool]
    """Whether the latest firmware differs from the `fw_ver` in `get_device_info()`."""
    release_note: Optional[str]
    """Release notes of the latest firmware."""
    fw_release_date: Optional[str]
    """Release date of the latest firmware."""
    fw_size: Optional[int]
    """Size of the firmware in bytes."""
    oem_id: Optional[str]
    """OEM id of the device."""
    upgrade_type: Optional[int]
    """Upgrade type, reported by the device as `type`."""
