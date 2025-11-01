from typing import List, Optional

from tapo import PowerStripPlugEnergyMonitoringHandler
from tapo.device_management_ext import DeviceManagementExt
from tapo.responses import DeviceInfoPowerStripResult, PowerStripPlugEnergyMonitoringResult

class PowerStripEnergyMonitoringHandler(DeviceManagementExt):
    """Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
    [P316M](https://www.tp-link.com/us/search/?q=P316M) devices.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""

    async def get_device_info(self) -> DeviceInfoPowerStripResult:
        """Returns *device info* as `DeviceInfoPowerStripResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PowerStripEnergyMonitoringHandler.get_device_info_json`.

        Returns:
            DeviceInfoPowerStripResult: Device info of Tapo P300, P304M, P306 and P316M. Superset of `DeviceInfoGenericResult`.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_child_device_list(
        self,
    ) -> List[PowerStripPlugEnergyMonitoringResult]:
        """Returns *child device list* as `List[PowerStripPlugEnergyMonitoringResult]`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PowerStripEnergyMonitoringHandler.get_child_device_list_json`.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_child_device_list_json(self) -> dict:
        """Returns *child device list* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_child_device_component_list_json(self) -> dict:
        """Returns *child device component list* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def plug(
        self,
        device_id: Optional[str] = None,
        nickname: Optional[str] = None,
        position: Optional[int] = None,
    ) -> PowerStripPlugEnergyMonitoringHandler:
        """Returns a `PowerStripPlugEnergyMonitoringHandler` for the device matching the provided `device_id`, `nickname`, or `position`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device
            position (Optional[str]): The Position of the device

        Returns:
            PowerStripPlugEnergyMonitoringHandler: Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
            [P316M](https://www.tp-link.com/us/search/?q=P316M) child plugs.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            power_strip = await client.p304("192.168.1.100")

            # Get a handler for the child device
            device = await power_strip.plug(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """
