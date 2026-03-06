from tapo.debug_ext import DebugExt
from tapo.device_management_ext import DeviceManagementExt
from tapo.on_off_ext import OnOffExt
from tapo.refresh_session_ext import RefreshSessionExt
from tapo.responses import DeviceInfoPlugResult, DeviceUsageResult

class PlugHandler(OnOffExt, DeviceManagementExt, RefreshSessionExt, DebugExt):
    """Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
    [P105](https://www.tapo.com/en/search/?q=P105) devices.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_device_info(self) -> DeviceInfoPlugResult:
        """Returns *device info* as `DeviceInfoPlugResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PlugHandler.get_device_info_json`.

        Returns:
            DeviceInfoPlugResult: Device info of Tapo P100 and P105.
            Superset of `GenericDeviceInfoResult`.
        """

    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time usage.
        """
