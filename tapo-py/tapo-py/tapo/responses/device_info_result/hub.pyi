from typing import Optional
from tapo.to_dict_ext import ToDictExt

class DeviceInfoHubResult(ToDictExt):
    """Device info of Tapo H100."""

    avatar: str
    device_id: str
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
    in_alarm_source: str
    in_alarm: bool
    overheated: bool
