from typing import Final

from tapo.to_dict_ext import ToDictExt

class WifiSecurity:
    """The security of a Wi-Fi network."""

    NONE: Final[WifiSecurity]
    """An open network, without a password."""
    Wep: Final[WifiSecurity]
    """WEP."""
    WpaPsk: Final[WifiSecurity]
    """WPA Personal."""
    Wpa2Psk: Final[WifiSecurity]
    """WPA2 Personal."""
    Wpa3Sae: Final[WifiSecurity]
    """WPA3 Personal."""
    Unknown: Final[WifiSecurity]
    """A security type that the library does not know. A network with it cannot be joined."""

class WifiNetwork(ToDictExt):
    """A Wi-Fi network that the device sees."""

    ssid: str
    """The network's name."""
    security: WifiSecurity
    """The security of the network. Pass it to `SetupHandler.join_wifi` to join this network."""
    signal_level: int
    """The signal strength as the device rates it, higher being stronger."""
    bssid: str
    """The MAC address of the access point, as upper-case hex without separators."""
    channel: int
    """The Wi-Fi channel."""
