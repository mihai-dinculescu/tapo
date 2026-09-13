from tapo.debug_ext import DebugExt
from tapo.responses import IrRemoteResult

class IrRemoteHandler(DebugExt):
    """Handler for the IR remotes paired with a
    [H110](https://www.tapo.com/en/search/?q=H110) hub.

    IR remotes are virtual child devices that are created by the Tapo app, either by
    picking an appliance from TP-Link's IR database or by learning the keys from a
    physical remote.
    """

    async def get_device_info(self) -> IrRemoteResult:
        """Returns *device info* as `IrRemoteResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `IrRemoteHandler.get_device_info_json`.

        Returns:
            IrRemoteResult: Device info of the IR remotes paired with a Tapo H110 hub.
        """

    async def send_ir_cmd_by_id(self, key_name: str) -> None:
        """Sends one of the IR keys stored on this remote.

        Args:
            key_name (str): the `name` of a key from this remote's `key_list`
        """
