from typing import Optional

class DeviceInfoPowerStripResult:
    """Device info of Tapo P300, P304M, P306 and P316M. Superset of `GenericDeviceInfoResult`."""

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

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
