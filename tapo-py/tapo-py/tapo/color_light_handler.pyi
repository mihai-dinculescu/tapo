from tapo.requests import Color, ColorLightSetDeviceInfoParams
from tapo.responses import DeviceInfoColorLightResult, DeviceUsageResult

class ColorLightHandler:
    """Handler for the [L530](https://www.tapo.com/en/search/?q=L530),
    [L535](https://www.tapo.com/en/search/?q=L535) and
    [L630](https://www.tapo.com/en/search/?q=L630) devices.
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

    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """

    async def get_device_info(self) -> DeviceInfoColorLightResult:
        """Returns *device info* as `DeviceInfoColorLightResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `ColorLightHandler.get_device_info_json`.

        Returns:
            DeviceInfoColorLightResult: Device info of Tapo L530, L535 and L630.
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

    def set(self) -> ColorLightSetDeviceInfoParams:
        """Returns a `ColorLightSetDeviceInfoParams` builder that allows
        multiple properties to be set in a single request.
        `ColorLightSetDeviceInfoParams.send` must be called at the end to apply the changes.

        Returns:
            ColorLightSetDeviceInfoParams: Builder that is used by the
            `ColorLightHandler.set` API to set multiple properties in a single request.
        """

    async def set_brightness(self, brightness: int) -> None:
        """Sets the *brightness* and turns *on* the device.

        Args:
            brightness (int): between 1 and 100
        """

    async def set_color(self, color: Color) -> None:
        """Sets the *color* and turns *on* the device.

        Args:
            color (Color): one of `tapo.Color` as defined in the Google Home app.
        """

    async def set_hue_saturation(self, hue: int, saturation: int) -> None:
        """Sets the *hue*, *saturation* and turns *on* the device.

        Args:
            hue (int): between 0 and 360
            saturation (int): between 1 and 100
        """

    async def set_color_temperature(self, color_temperature: int) -> None:
        """Sets the *color temperature* and turns *on* the device.

        Args:
            color_temperature (int): between 2500 and 6500
        """
