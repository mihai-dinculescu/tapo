from typing import Optional

from tapo.responses.device_info_result.default_state import DefaultStateType
from tapo.to_dict_ext import ToDictExt

class DeviceInfoColorLightResult(ToDictExt):
    """Device info of Tapo L530, L535 and L630."""

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
    color_temp: int
    default_states: DefaultColorLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    dynamic_light_effect_enable: bool
    dynamic_light_effect_id: Optional[str]
    hue: Optional[int]
    overheated: bool
    saturation: Optional[int]

class DefaultColorLightState(ToDictExt):
    """Color Light Default State."""

    type: DefaultStateType
    state: ColorLightState

class ColorLightState(ToDictExt):
    """Color Light State."""

    brightness: int
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: int
