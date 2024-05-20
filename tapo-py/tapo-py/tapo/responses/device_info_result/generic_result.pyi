from typing import Optional

class DeviceInfoGenericResult:
    """Device info of a Generic Tapo device."""

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
    """The time in seconds this device has been ON since the last state change (ON/OFF)."""
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
