from typing import Optional
from tapo.to_dict_ext import ToDictExt

class DeviceInfoBasicResult(ToDictExt):
    """Basic device info of a Tapo device."""

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
    device_on: Optional[bool]
    on_time: Optional[int]
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]
