from typing import List, Optional

from tapo.responses.device_info_result.default_state import DefaultStateType
from tapo.to_dict_ext import ToDictExt

class DeviceInfoRgbLightStripResult(ToDictExt):
    """Device info of Tapo L900."""

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
    color_temp_range: List[int]
    color_temp: int
    default_states: DefaultRgbLightStripState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    hue: Optional[int]
    overheated: bool
    saturation: Optional[int]

class DefaultRgbLightStripState(ToDictExt):
    """RGB Light Strip Default State."""

    type: DefaultStateType
    state: RgbLightStripState

class RgbLightStripState(ToDictExt):
    """RGB Light Strip State."""

    brightness: Optional[int]
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: Optional[int]
