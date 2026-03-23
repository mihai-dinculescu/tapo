from typing import Optional

from tapo.responses import DefaultBrightnessState, DefaultPowerType
from tapo.responses.device_info_result.device_info_ext import DeviceInfoSmartExt
from tapo.to_dict_ext import ToDictExt

class DeviceInfoLightResult(DeviceInfoSmartExt, ToDictExt):
    """Device info of Tapo L510, L520 and L610."""

    brightness: int
    default_states: DefaultLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    device_on: bool
    nickname: str
    on_time: Optional[int]
    """The time in seconds this device has been ON since the last state change (On/Off). On v2 hardware this is always None."""
    overheated: bool

class DefaultLightState(ToDictExt):
    """Light Default State."""

    brightness: DefaultBrightnessState
    re_power_type: Optional[DefaultPowerType]
