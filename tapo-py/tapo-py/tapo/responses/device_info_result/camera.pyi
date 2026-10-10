from typing import Optional

from tapo.to_dict_ext import ToDictExt

class DeviceInfoCameraResult(ToDictExt):
    """Device info of Tapo cameras (C210, C220, C225, C325WB, C520WS, TC40, TC70)."""

    avatar: str
    device_id: str
    device_info: str
    device_name: str
    fw_ver: str
    has_set_location_info: bool
    hw_id: str
    hw_ver: str
    latitude: Optional[float]
    longitude: Optional[float]
    mac: str
    model: str
    nickname: str
    no_rtsp_constrain: Optional[bool]
    """Whether RTSP streaming is available without restrictions.
    Only present on some models (e.g. C220, C225, C720)."""
    region: Optional[str]
    type: str
