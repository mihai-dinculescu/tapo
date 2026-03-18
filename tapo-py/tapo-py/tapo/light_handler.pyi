from tapo.debug_ext import DebugExt
from tapo.device_management_ext import DeviceManagementExt
from tapo.on_off_ext import OnOffExt
from tapo.refresh_session_ext import RefreshSessionExt
from tapo.responses import DeviceInfoLightResult, DeviceUsageEnergyMonitoringResult

class LightHandler(OnOffExt, DeviceManagementExt, RefreshSessionExt, DebugExt):
    """Handler for the [L510](https://www.tapo.com/en/search/?q=L510),
    [L520](https://www.tapo.com/en/search/?q=L520) and
    [L610](https://www.tapo.com/en/search/?q=L610) devices."""

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_device_info(self) -> DeviceInfoLightResult:
        """Returns *device info* as `DeviceInfoLightResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `LightHandler.get_device_info_json`.

        Returns:
            DeviceInfoLightResult: Device info of Tapo L510, L520 and L610.
        """

    async def get_device_usage(self) -> DeviceUsageEnergyMonitoringResult:
        """Returns *device usage* as `DeviceUsageEnergyMonitoringResult`.

        Returns:
            DeviceUsageEnergyMonitoringResult:
            Contains the time usage, the power consumption, and the energy savings of the device.
        """

    async def set_brightness(self, brightness: int) -> None:
        """Sets the *brightness* and turns *on* the device.

        Args:
            brightness (int): between 1 and 100
        """
