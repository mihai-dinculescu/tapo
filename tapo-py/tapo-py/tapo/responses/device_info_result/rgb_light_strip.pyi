from typing import List, Optional

from tapo.responses.device_info_result.default_state import DefaultStateType
from tapo.responses.device_info_result.device_info_ext import DeviceInfoSmartExt
from tapo.to_dict_ext import ToDictExt

class DeviceInfoRgbLightStripResult(DeviceInfoSmartExt, ToDictExt):
    """Device info of Tapo L900."""

    brightness: int
    color_temp_range: List[int]
    color_temp: int
    default_states: DefaultRgbLightStripState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    device_on: bool
    hue: Optional[int]
    nickname: str
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
