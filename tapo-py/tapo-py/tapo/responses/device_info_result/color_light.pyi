from typing import Optional

from tapo.responses.device_info_result.default_state import DefaultStateType
from tapo.responses.device_info_result.device_info_ext import DeviceInfoSmartExt
from tapo.to_dict_ext import ToDictExt

class DeviceInfoColorLightResult(DeviceInfoSmartExt, ToDictExt):
    """Device info of Tapo L530, L535 and L630."""

    brightness: int
    color_temp: int
    default_states: DefaultColorLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    device_on: bool
    dynamic_light_effect_enable: bool
    dynamic_light_effect_id: Optional[str]
    hue: Optional[int]
    nickname: str
    on_time: Optional[int]
    """The time in seconds this device has been ON since the last state change (On/Off). On v2 hardware this is always None."""
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
