from tapo.responses import PowerStripPlugResult

class PowerStripPlugHandler:
    """PowerStripHandler: Handler for the [P300](https://www.tapo.com/en/search/?q=P300),
    [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
    [P316M](https://www.tp-link.com/us/search/?q=P316M) child plugs.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def on(self) -> None:
        """Turns *on* the device."""

    async def off(self) -> None:
        """Turns *off* the device."""

    async def get_device_info(self) -> PowerStripPlugResult:
        """Returns *device info* as `PowerStripPlugResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PlugHandler.get_device_info_json`.

        Returns:
            PowerStripPlugResult: P300, P304M and P316M power strip child plugs.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """
