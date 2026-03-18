from typing import Optional

from tapo.responses import DefaultBrightnessState, DefaultPowerType
from tapo.to_dict_ext import ToDictExt

class DeviceInfoLightResult(ToDictExt):
    """Device info of Tapo L510, L520 and L610."""

    avatar: str
    device_id: str
    device_on: bool
    fw_id: str
    fw_ver: str
    has_set_location_info: bool
    hw_id: str
    hw_ver: str
    ip: str
    lang: str
    latitude: Optional[float]
    longitude: Optional[float]
    mac: str
    model: str
    nickname: str
    oem_id: str
    on_time: int
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    region: Optional[str]
    rssi: int
    signal_level: int
    specs: str
    ssid: str
    time_diff: Optional[int]
    type: str

    # Unique to this device
    brightness: int
    default_states: DefaultLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    overheated: bool

class DefaultLightState(ToDictExt):
    """Light Default State."""

    brightness: DefaultBrightnessState
    re_power_type: Optional[DefaultPowerType]
