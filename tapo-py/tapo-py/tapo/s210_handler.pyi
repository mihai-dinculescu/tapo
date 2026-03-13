from tapo.debug_ext import DebugExt
from tapo.on_off_ext import OnOffExt
from tapo.responses import DeviceUsageResult, S210Result

class S210Handler(OnOffExt, DebugExt):
    """Handler for the [S210](https://www.tapo.com/en/search/?q=S210) devices."""

    async def get_device_info(self) -> S210Result:
        """Returns *device info* as `S210Result`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `S210Handler.get_device_info_json`.

        Returns:
            S210Result: Device info of Tapo S210 light switch.
        """

    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time usage of the device.
        """
