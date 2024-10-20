from typing import Optional

from tapo.responses.device_info_result.default_state import DefaultStateType

class DeviceInfoColorLightResult:
    """Device info of Tapo L530, L535 and L630. Superset of `GenericDeviceInfoResult`."""

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
    color_temp: int
    default_states: DefaultColorLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    dynamic_light_effect_enable: bool
    dynamic_light_effect_id: Optional[str]
    hue: Optional[int]
    overheated: bool
    saturation: Optional[int]

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DefaultColorLightState:
    """Color Light Default State."""

    type: DefaultStateType
    state: ColorLightState

class ColorLightState:
    """Color Light State."""

    brightness: int
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: int
