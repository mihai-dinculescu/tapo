from typing import List, Optional

from tapo.responses.device_info_result.default_state import DefaultStateType
from tapo.requests.set_device_info.lighting_effect import LightingEffect

class DeviceInfoRgbicLightStripResult:
    """Device info of Tapo L920 and L930. Superset of `GenericDeviceInfoResult`."""

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
    device_on: bool
    on_time: int
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    # Unique to this device
    brightness: int
    color_temp_range: List[int]
    color_temp: int
    default_states: DefaultRgbicLightStripState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    hue: Optional[int]
    overheated: bool
    saturation: Optional[int]

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DefaultRgbicLightStripState:
    """RGB IC Light Strip Default State."""

    type: DefaultStateType
    state: RgbicLightStripState

class RgbicLightStripState:
    """RGB IC Light Strip State."""

    brightness: Optional[int]
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: Optional[int]
    lighting_effect: Optional[LightingEffect]
