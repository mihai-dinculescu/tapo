from typing import Final, Optional

from tapo.to_dict_ext import ToDictExt

class CloudConnectionStatus:
    """The status of the device's connection to the Tapo cloud."""

    Connected: Final[CloudConnectionStatus]
    """The device is connected to the Tapo cloud."""
    Disconnected: Final[CloudConnectionStatus]
    """The device is not connected to the Tapo cloud."""
    Connecting: Final[CloudConnectionStatus]
    """The device is trying to connect to the Tapo cloud."""
    Disconnecting: Final[CloudConnectionStatus]
    """The device is disconnecting from the Tapo cloud."""
    Unknown: Final[CloudConnectionStatus]
    """The device reported a status that the library does not recognize."""

class CloudConnectionStateResult(ToDictExt):
    """Contains the state of the device's connection to the Tapo cloud.

    Only a `status` of `CloudConnectionStatus.Connected` means that the device reaches
    the Tapo cloud. A device that has lost its internet connection keeps trying to
    reconnect, so it reports `CloudConnectionStatus.Connecting` for as long as the
    connection is down.
    """

    status: CloudConnectionStatus
    """Status of the connection to the Tapo cloud."""
    code: Optional[int]
    """Status code that the device reported. `None` unless `status` is `Unknown`."""
