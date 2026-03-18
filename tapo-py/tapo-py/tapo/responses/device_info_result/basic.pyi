from typing import Optional
from tapo.to_dict_ext import ToDictExt

class DeviceInfoBasicResult(ToDictExt):
    """Basic device info of a Tapo device."""

    avatar: str
    device_id: str
    device_on: Optional[bool]
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
    on_time: Optional[int]
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    region: Optional[str]
    rssi: int
    signal_level: int
    specs: str
    ssid: str
    time_diff: Optional[int]
    type: str
