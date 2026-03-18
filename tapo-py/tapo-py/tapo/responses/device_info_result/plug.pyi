from typing import Optional, Union

from tapo.responses.device_info_result.default_plug_state import Custom, LastStates
from tapo.to_dict_ext import ToDictExt

class DeviceInfoPlugResult(ToDictExt):
    """Device info of Tapo P100 and P105."""

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
    default_states: Union[LastStates, Custom]
