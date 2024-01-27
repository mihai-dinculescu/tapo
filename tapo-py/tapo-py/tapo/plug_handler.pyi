from typing import Optional
from .types import DefaultStateType, DeviceUsageResult

class PlugHandler:
    """Handler for the [P100](https://www.tapo.com/en/search/?q=P100) & [P105](https://www.tapo.com/en/search/?q=P105) devices."""

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """
    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""
    async def on(self) -> None:
        """Turns *on* the device."""
    async def off(self) -> None:
        """Turns *off* the device."""
    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """
    async def get_device_info(self) -> DeviceInfoPlugResult:
        """Returns *device info* as `DeviceInfoPlugResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PlugHandler.get_device_info_json`.

        Returns:
            DeviceInfoPlugResult: Device info of Tapo P100, P105, P110 and P115.
            Superset of `GenericDeviceInfoResult`.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """
    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time usage.
        """

class DeviceInfoPlugResult:
    """Device info of Tapo P100, P105, P110 and P115. Superset of `GenericDeviceInfoResult`."""

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
    """The time in seconds this device has been ON since the last state change (ON/OFF)."""
    overheated: bool
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    # Unique to this device
    default_states: PlugDefaultState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class PlugDefaultState:
    """Plug Default State."""

    type: DefaultStateType
    state: PlugState

class PlugState:
    """Plug State."""

    on: Optional[bool]
