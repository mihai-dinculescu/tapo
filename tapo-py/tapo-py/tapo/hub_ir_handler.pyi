from typing import List, Optional, Union

from tapo import IrRemoteHandler
from tapo.debug_ext import DebugExt
from tapo.device_handler_ext import DeviceHandlerExt
from tapo.device_management_ext import DeviceManagementExt
from tapo.hub_alarm_ext import HubAlarmExt
from tapo.hub_children_ext import HubChildrenExt
from tapo.responses import (
    DeviceInfoHubResult,
    IrRemoteResult,
    KE100Result,
    OtherResult,
    S200Result,
    S210Result,
    T100Result,
    T110Result,
    T300Result,
    T31XResult,
)

class HubIrHandler(DeviceHandlerExt, HubAlarmExt, HubChildrenExt, DeviceManagementExt, DebugExt):
    """Handler for the [H110](https://www.tapo.com/en/search/?q=H110) devices.

    Example:
        ```python
        # Connect to the hub
        client = ApiClient("tapo-username@example.com", "tapo-password")
        hub = await client.h110("192.168.1.100")

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
        try `HubIrHandler.get_device_info_json`.

        Returns:
            DeviceInfoHubResult: Device info of Tapo H100 and H110.
        """

    async def get_child_device_list(
        self,
    ) -> List[
        Union[
            IrRemoteResult,
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
        """Returns *child device list* as `List[IrRemoteResult | KE100Result | S200Result | S210Result | T100Result | T110Result | T300Result | T31XResult | OtherResult]`.
        It is not guaranteed to contain all the properties returned from the Tapo API
        or to support all the possible devices connected to the hub.
        If the deserialization fails, or if a property that you care about it's not present,
        try `HubIrHandler.get_child_device_list_json`.

        Returns:
            List[IrRemoteResult | KE100Result | S200Result | S210Result | T100Result | T110Result | T300Result | T31XResult | OtherResult]: The child devices paired to the hub.
        """

    async def ir_remote(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> IrRemoteHandler:
        """Returns an `IrRemoteHandler` for the device matching the provided `device_id` or `nickname`.

        IR remotes must be configured in the Tapo app first.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            IrRemoteHandler: Handler for the IR remotes paired with a
            [H110](https://www.tapo.com/en/search/?q=H110) hub.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h110("192.168.1.100")

            # Get a handler for the child device
            device = await hub.ir_remote(nickname="Living Room TV")

            # Send one of the keys stored on the remote
            await device.send_key_by_name("POWER")
            ```
        """

    async def ir_remote_unchecked(self, device_id: str) -> IrRemoteHandler:
        """Returns an `IrRemoteHandler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `HubIrHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `HubIrHandler.ir_remote`.
        """
