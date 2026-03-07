from typing import Optional
from tapo.to_dict_ext import ToDictExt

class DeviceInfoHubResult(ToDictExt):
    """Device info of Tapo H100. Superset of `GenericDeviceInfoResult`."""

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
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    # Unique to this device
    in_alarm_source: str
    in_alarm: bool
    overheated: bool
