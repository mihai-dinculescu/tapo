from tapo.debug_ext import DebugExt
from tapo.on_off_ext import OnOffExt
from tapo.refresh_session_ext import RefreshSessionExt
from tapo.responses import DeviceInfoGenericResult

class GenericDeviceHandler(OnOffExt, RefreshSessionExt, DebugExt):
    """Handler for generic devices. It provides the functionality common to
    all Tapo [devices](https://www.tapo.com/en/).

    If you'd like to propose support for a device that isn't currently supported,
    please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues) to start the conversation.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_device_info(self) -> DeviceInfoGenericResult:
        """Returns *device info* as `DeviceInfoGenericResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `GenericDeviceHandler.get_device_info_json`.

        Returns:
            DeviceInfoGenericResult: Device info of a Generic Tapo device.
        """
