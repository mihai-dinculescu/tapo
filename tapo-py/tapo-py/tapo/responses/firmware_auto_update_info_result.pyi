from tapo.to_dict_ext import ToDictExt

class FirmwareAutoUpdateInfoResult(ToDictExt):
    """Contains the device's automatic firmware update settings."""

    enabled: bool
    """Whether the device installs firmware updates automatically."""
    start_min: int
    """Start of the update window, in minutes after midnight in the device's timezone
    (`120` is 2:00 AM)."""
    window_min: int
    """Length of the update window, in minutes. The Tapo app always uses `120`."""
