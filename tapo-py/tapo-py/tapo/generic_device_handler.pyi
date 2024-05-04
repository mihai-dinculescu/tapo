from tapo.responses import DeviceInfoGenericResult

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
