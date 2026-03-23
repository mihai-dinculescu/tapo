from typing import Optional

class DeviceInfoSmartExt:
    """Properties common to all device info results in the Smart device family (plugs, lights, power strips, etc.)."""

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
    oem_id: str
    region: Optional[str]
    rssi: int
    signal_level: int
    specs: str
    ssid: str
    time_diff: Optional[int]
    type: str
