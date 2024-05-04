from typing import Optional

from tapo.responses import DefaultStateType

class DeviceInfoPlugResult:
    """Device info of Tapo P100, P105, P110 and P115. Superset of `GenericDeviceInfoResult`."""

    device_id: str
    type: str
    model: str
    hw_id: str
    hw_ver: str
    fw_id: str
    fw_ver: str
    oem_id: str
    mac: str
    ip: str
    ssid: str
    signal_level: int
    rssi: int
    specs: str
    lang: str
    device_on: bool
    on_time: int
    """The time in seconds this device has been ON since the last state change (ON/OFF)."""
    overheated: bool
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    # Unique to this device
    default_states: DefaultPlugState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DefaultPlugState:
    """Default Plug State."""

    type: DefaultStateType
    state: PlugState

class PlugState:
    """Plug State."""

    on: Optional[bool]
