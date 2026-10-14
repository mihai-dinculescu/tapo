from typing import List, Union

from tapo.debug_ext import DebugExt
from tapo.device_handler_ext import DeviceHandlerExt
from tapo.device_management_ext import DeviceManagementExt
from tapo.hub_alarm_ext import HubAlarmExt
from tapo.hub_children_ext import HubChildrenExt
from tapo.responses import (
    DeviceInfoHubResult,
    KE100Result,
    OtherResult,
    S200Result,
    S210Result,
    T100Result,
    T110Result,
    T300Result,
    T31XResult,
)

class HubHandler(DeviceHandlerExt, HubAlarmExt, HubChildrenExt, DeviceManagementExt, DebugExt):
    """Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices.

    Example:
        ```python
        # Connect to the hub
        client = ApiClient("tapo-username@example.com", "tapo-password")
        hub = await client.h100("192.168.1.100")

        # Get a handler for a child device
        device = await hub.t31x(device_id="0000000000000000000000000000000000000000")

        # Get the device info of the child device
        device_info = await device.get_device_info()
        print(f"Device info: {device_info.to_dict()}")
        ```
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_device_info(self) -> DeviceInfoHubResult:
        """Returns *device info* as `DeviceInfoHubResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `HubHandler.get_device_info_json`.

        Returns:
            DeviceInfoHubResult: Device info of Tapo H100 and H110.
        """

    async def get_child_device_list(
        self,
    ) -> List[
        Union[
            KE100Result,
            S200Result,
            S210Result,
            T100Result,
            T110Result,
            T300Result,
            T31XResult,
            OtherResult,
        ]
    ]:
        """Returns *child device list* as `List[KE100Result | S200Result | S210Result | T100Result | T110Result | T300Result | T31XResult | OtherResult]`.
        It is not guaranteed to contain all the properties returned from the Tapo API
        or to support all the possible devices connected to the hub.
        If the deserialization fails, or if a property that you care about it's not present,
        try `HubHandler.get_child_device_list_json`.

        Returns:
            List[KE100Result | S200Result | S210Result | T100Result | T110Result | T300Result | T31XResult | OtherResult]: The child devices paired to the hub.
        """
