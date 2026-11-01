from typing import Optional
from tapo.to_dict_ext import ToDictExt

class FirmwareUpgradeState(ToDictExt):
    """Contains the download and upgrade progress of an upgrade running on the device."""

    status: Optional[int]
    """Status of the upgrade, reported by the device."""
    download_progress: Optional[int]
    """Download progress in percent."""
    reboot_time: Optional[int]
    """Time in seconds the device spends rebooting during the upgrade."""
    upgrade_time: Optional[int]
    """Time in seconds the upgrade takes."""
    auto_upgrade: Optional[bool]
    """Whether the device upgrades its firmware automatically."""
