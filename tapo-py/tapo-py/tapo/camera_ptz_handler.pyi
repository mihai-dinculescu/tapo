from tapo.debug_ext import DebugExt
from tapo.refresh_session_ext import RefreshSessionExt
from tapo.responses import DeviceInfoCameraResult, Preset, RtspStreamUrl

class CameraPtzHandler(RefreshSessionExt, DebugExt):
    """Handler for Tapo cameras with PTZ, such as the
    [C210](https://www.tapo.com/en/search/?q=C210),
    [C220](https://www.tapo.com/en/search/?q=C220),
    [C225](https://www.tapo.com/en/search/?q=C225),
    [C325WB](https://www.tapo.com/en/search/?q=C325WB),
    [C520WS](https://www.tapo.com/en/search/?q=C520WS),
    [TC40](https://www.tapo.com/en/search/?q=TC40),
    and [TC70](https://www.tapo.com/en/search/?q=TC70).
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_device_info(self) -> DeviceInfoCameraResult:
        """Returns *device info* as `DeviceInfoCameraResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `CameraPtzHandler.get_device_info_json`.

        Returns:
            DeviceInfoCameraResult: Device info of Tapo PTZ cameras.
        """

    async def get_rtsp_stream_url(self, username: str, password: str) -> RtspStreamUrl:
        """Returns the RTSP stream URLs for the camera.

        The credentials are the **camera account** credentials set in the Tapo app
        (Camera Settings > Advanced Settings > Camera Account), not the TP-Link cloud account credentials.
        They will be URL-encoded automatically.

        Args:
            username (str): The camera account username.
            password (str): The camera account password.

        Returns:
            RtspStreamUrl: The HD and SD RTSP stream URLs.
        """

    async def pan_tilt(self, pan: int, tilt: int) -> None:
        """Moves the camera by the given pan and tilt values.

        Positive `pan` moves right, negative moves left. `0` will not move on this axis.
        Positive `tilt` moves up, negative moves down. `0` will not move on this axis.

        If unsure of the value, `10` for both `pan` and `tilt` are good values for small nudges.

        Args:
            pan (int): The pan step.
            tilt (int): The tilt step.
        """

    async def save_preset(self, name: str) -> None:
        """Saves the current camera position as a named preset.

        Args:
            name (str): The preset name.
        """

    async def goto_preset(self, id: str) -> None:
        """Moves the camera to a saved preset position by its ID.

        Args:
            id (str): The preset ID.
        """

    async def delete_preset(self, id: str) -> None:
        """Deletes a preset by its ID.

        Args:
            id (str): The preset ID.
        """

    async def get_presets(self) -> list[Preset]:
        """Returns the list of saved PTZ presets.

        Returns:
            list[Preset]: The list of presets.
        """
