from typing import List, Optional

from tapo.responses.device_info_result.default_state import DefaultStateType
from tapo.requests.set_device_info.lighting_effect import LightingEffect
from tapo.to_dict_ext import ToDictExt

class DeviceInfoRgbicLightStripResult(ToDictExt):
    """Device info of Tapo L920 and L930."""

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
    region: Optional[str]
    rssi: int
    signal_level: int
    specs: str
    ssid: str
    time_diff: Optional[int]
    type: str

    # Unique to this device
    brightness: int
    color_temp_range: List[int]
    color_temp: int
    default_states: DefaultRgbicLightStripState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    hue: Optional[int]
    overheated: bool
    saturation: Optional[int]

class DefaultRgbicLightStripState(ToDictExt):
    """RGB IC Light Strip Default State."""

    type: DefaultStateType
    state: RgbicLightStripState

class RgbicLightStripState(ToDictExt):
    """RGB IC Light Strip State."""

    brightness: Optional[int]
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: Optional[int]
    lighting_effect: Optional[LightingEffect]
