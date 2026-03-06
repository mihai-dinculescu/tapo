from .responses import Component

class DebugExt:
    """Extension class for debug capabilities like `get_device_info_json`."""

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_component_list(self) -> list[Component]:
        """Returns the *component list* of the device.

        Returns:
            list[Component]: The list of components supported by the device.
        """
