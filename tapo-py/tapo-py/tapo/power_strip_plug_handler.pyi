from tapo.debug_ext import DebugExt
from tapo.on_off_ext import OnOffExt
from tapo.responses import PowerStripPlugResult

class PowerStripPlugHandler(OnOffExt, DebugExt):
    """Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
    [P306](https://www.tp-link.com/us/search/?q=P306) child plugs.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_device_info(self) -> PowerStripPlugResult:
        """Returns *device info* as `PowerStripPlugResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PowerStripPlugHandler.get_device_info_json`.

        Returns:
            PowerStripPlugResult: P300 and P306 power strip child plugs.
        """
