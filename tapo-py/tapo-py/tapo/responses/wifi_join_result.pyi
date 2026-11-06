from typing import Final

class WifiJoinResult:
    """How far the device has confirmed joining a Wi-Fi network."""

    Sent: Final[WifiJoinResult]
    """The device took the network's details and left its setup network without
    saying whether it could join. Find it on the network that it was given, for
    example with `ApiClient.discover_devices`."""
    Connected: Final[WifiJoinResult]
    """The device said that it joined the network, which it does when it is set up
    over Bluetooth. Find it on that network, for example with
    `ApiClient.discover_devices`."""
