from typing import Union

from tapo.responses.device_info_result.default_plug_state import Custom, LastStates
from tapo.responses.device_info_result.device_info_ext import DeviceInfoExt
from tapo.to_dict_ext import ToDictExt

class DeviceInfoPlugResult(DeviceInfoExt, ToDictExt):
    """Device info of Tapo P100 and P105."""

    default_states: Union[LastStates, Custom]
    device_on: bool
    nickname: str
    on_time: int
    """The time in seconds this device has been ON since the last state change (On/Off)."""
