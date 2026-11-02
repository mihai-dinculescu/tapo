from typing import Optional

from tapo.to_dict_ext import ToDictExt

class LatestFirmwareResult(ToDictExt):
    """Contains the newest firmware that the device can update to, as the device reports it.

    When no update is available, `type` is `0` and the other fields are empty or describe
    the firmware the device is running.
    """

    fw_size: Optional[int]
    """A size the device reports with the update. It is not the size of the
    firmware image: every non-zero size in python-kasa's fixtures is `786432`.
    `0` when no update is available, and `None` on devices that do not send it."""
    fw_ver: str
    """Version of the newest firmware. When no update is available, the version the
    device is running."""
    hw_id: str
    """Hardware ID that the update is for. Empty when no update is available."""
    need_to_upgrade: bool
    """`True` when an update is available. In python-kasa's device fixtures, it is
    `True` exactly when `type` is non-zero."""
    oem_id: str
    """OEM ID that the update is for. Empty when no update is available."""
    release_date: str
    """Release date of the newest firmware, as text. Empty when no update is available."""
    release_note: str
    """Release notes of the newest firmware. Empty when no update is available."""
    type: int
    """`0` when no update is available, and non-zero when one is. python-kasa's
    device fixtures hold `1`, `2` and `3`."""
