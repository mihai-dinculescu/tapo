from typing import Optional

from tapo.to_dict_ext import ToDictExt

class FirmwareUpdateStateResult(ToDictExt):
    """Contains the progress of a firmware update, as the device reports it."""

    auto_upgrade: Optional[bool]
    """Whether the device reports the update as an automatic one. `None` on devices
    that do not report it."""
    download_progress: int
    """Download progress in percent (0-100)."""
    reboot_time: int
    """Time the device takes to reboot after flashing, in seconds."""
    status: int
    """State of the update. python-kasa reads `0` as idle or finished, `2` as
    downloading, `3` as flashing, and a negative value as a failed update. Other
    values occur: a T110 in python-kasa's fixtures reports `4`."""
    upgrade_time: int
    """Time the device takes to flash the firmware, in seconds."""
