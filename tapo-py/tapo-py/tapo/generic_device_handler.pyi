from typing import Optional

class GenericDeviceHandler:
    """Handler for generic devices. It provides the functionality common to
    all Tapo [devices](https://www.tapo.com/en/).
    """

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
    async def get_device_info(self) -> DeviceInfoGenericResult:
        """Returns *device info* as `DeviceInfoGenericResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `GenericDeviceHandler.get_device_info_json`.

        Returns:
            DeviceInfoGenericResult: Device info of a Generic Tapo device.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

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
    overheated: bool
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
