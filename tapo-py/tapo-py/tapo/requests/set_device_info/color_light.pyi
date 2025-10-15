from typing import Union

from tapo.color_light_handler import ColorLightHandler
from tapo.requests import Color
from tapo.rgb_light_strip_handler import RgbLightStripHandler

class ColorLightSetDeviceInfoParams:
    """Builder that is used by the `ColorLightHandler.set` API to set
    multiple properties in a single request.
    """

    def on(self) -> ColorLightSetDeviceInfoParams:
        """Turns *on* the device.
        `ColorLightSetDeviceInfoParams.send` must be called at the end to apply the changes.
        """

    def off(self) -> ColorLightSetDeviceInfoParams:
        """Turns *off* the device.
        `ColorLightSetDeviceInfoParams.send` must be called at the end to apply the changes.
        """

    def brightness(self, brightness: int) -> ColorLightSetDeviceInfoParams:
        """Sets the *brightness*.
        `ColorLightSetDeviceInfoParams.send` must be called at the end to apply the changes.
        The device will also be turned *on*, unless `ColorLightSetDeviceInfoParams.off` is called.

        Args:
            brightness (int): between 1 and 100
        """

    def color(self, color: Color) -> ColorLightSetDeviceInfoParams:
        """Sets the *color*.
        `ColorLightSetDeviceInfoParams.send` must be called at the end to apply the changes.
        The device will also be turned *on*, unless `ColorLightSetDeviceInfoParams.off` is called.

        Args:
            color (Color): one of `tapo.Color` as defined in the Google Home app.
        """

    def hue_saturation(self, hue: int, saturation: int) -> ColorLightSetDeviceInfoParams:
        """Sets the *hue* and *saturation*.
        `ColorLightSetDeviceInfoParams.send` must be called at the end to apply the changes.
        The device will also be turned *on*, unless `ColorLightSetDeviceInfoParams.off` is called.

        Args:
            hue (int): between 0 and 360
            saturation (int): between 1 and 100
        """

    def color_temperature(self, color_temperature: int) -> ColorLightSetDeviceInfoParams:
        """
        Sets the *color temperature*.
        `ColorLightSetDeviceInfoParams.send` must be called at the end to apply the changes.
        The device will also be turned *on*, unless `ColorLightSetDeviceInfoParams.off` is called.

        Args:
            color_temperature (int): between 2500 and 6500
        """

    async def send(self, handler: Union[ColorLightHandler, RgbLightStripHandler]) -> None:
        """Performs a request to apply the changes to the device.

        Args:
            handler (`ColorLightHandler` | `RgbLightStripHandler`)
        """
