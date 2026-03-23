from typing import Optional

from tapo.to_dict_ext import ToDictExt

class DeviceInfoBasicResult(ToDictExt):
    """Basic device info of a Tapo device."""

    avatar: str
    device_id: str
    fw_ver: str
    has_set_location_info: bool
    hw_ver: str
    latitude: Optional[float]
    longitude: Optional[float]
    mac: str
    model: str
    nickname: Optional[str]
    oem_id: str
    region: Optional[str]
    type: str
