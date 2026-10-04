from typing import List, Optional, Union

from tapo import HubHandler, IrRemoteHandler
from tapo.responses import (
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

class HubIrHandler(HubHandler):
    """Handler for the [H110](https://www.tapo.com/en/search/?q=H110) devices.

    It supports everything that `HubHandler` does, and also handles the IR remotes
    paired with the hub through `HubIrHandler.ir_remote`.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_child_device_list(  # type: ignore[override]
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
            await device.send_ir_cmd_by_id("POWER")
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
