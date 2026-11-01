from typing import Final

class CloudConnectionState:
    """Whether the device is connected to the Tapo Cloud."""

    Connected: Final[CloudConnectionState]
    """Connected to the Tapo Cloud."""

    Disconnected: Final[CloudConnectionState]
    """Disconnected from the Tapo Cloud."""

    Connecting: Final[CloudConnectionState]
    """Connecting to the Tapo Cloud."""

    Disconnecting: Final[CloudConnectionState]
    """Disconnecting from the Tapo Cloud."""

    Unknown: Final[CloudConnectionState]
    """A state code this library does not know."""
