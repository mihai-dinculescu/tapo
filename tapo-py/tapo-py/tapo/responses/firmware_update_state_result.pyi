from typing import Final, Optional

from tapo.to_dict_ext import ToDictExt

class FirmwareUpdateStatus:
    """The status of a firmware update."""

    Idle: Final[FirmwareUpdateStatus]
    """No firmware update is in progress."""
    Downloading: Final[FirmwareUpdateStatus]
    """The device is downloading the update."""
    Downloaded: Final[FirmwareUpdateStatus]
    """The update is downloaded and installs next.

    The device installs the update and reboots, which takes about `install_time_s`
    plus `reboot_time_s`, and does not answer while it reboots. An update that the
    device downloaded by itself for an automatic update stays in this state until the
    device installs it. For a hub child device, the hub can report this state when the
    child device already runs the new firmware."""
    AwaitingTransfer: Final[FirmwareUpdateStatus]
    """The hub has downloaded the update for this child device and has not sent it to
    the child device yet. The hub sends it once it can reach the child device, so a
    child device that is offline, for example with a flat battery, stays in this
    state."""
    Failed: Final[FirmwareUpdateStatus]
    """The update failed. The Tapo app reads `code` `-1001` as a failed download,
    `-1002` as a failed installation check, `-1003` as a failed transfer from the hub
    to the child device, and `-1004` as a battery too low to update."""
    Unknown: Final[FirmwareUpdateStatus]
    """The device reported a status that the library does not recognize."""

class FirmwareUpdateStateResult(ToDictExt):
    """Contains the state of a firmware update on the device."""

    status: FirmwareUpdateStatus
    """Status of the firmware update."""
    download_progress: Optional[int]
    """Download progress in percent (0-100). `None` unless `status` is `Downloading`."""
    install_time_s: Optional[int]
    """Time the device takes to install the update, in seconds. `None` unless `status`
    is `Downloaded`."""
    reboot_time_s: Optional[int]
    """Time the device takes to reboot after installing the update, in seconds. `None`
    unless `status` is `Downloaded`."""
    code: Optional[int]
    """Status code that the device reported. `None` unless `status` is `Failed` or
    `Unknown`."""
