from typing import Optional

from tapo.to_dict_ext import ToDictExt

class DeviceInfoCameraHubResult(ToDictExt):
    """Device info of Tapo camera hubs (H200, H500)."""

    avatar: str
    bind_status: bool
    child_num: int
    device_id: str
    device_info: str
    device_name: str
    fw_ver: str
    has_set_location_info: bool
    hw_id: str
    hw_ver: str
    ip: str
    latitude: Optional[int]
    longitude: Optional[int]
    mac: str
    model: str
    nickname: str
    oem_id: str
    product_name: str
    region: Optional[str]
    status: str
    type: str
