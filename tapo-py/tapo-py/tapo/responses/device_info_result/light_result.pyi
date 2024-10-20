from typing import Optional

from tapo.responses import DefaultBrightnessState, DefaultPowerType

class DeviceInfoLightResult:
    """Device info of Tapo L510, L520 and L610. Superset of `GenericDeviceInfoResult`."""

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
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    # Unique to this device
    brightness: int
    default_states: DefaultLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    overheated: bool

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DefaultLightState:
    """Light Default State."""

    brightness: DefaultBrightnessState
    re_power_type: Optional[DefaultPowerType]
