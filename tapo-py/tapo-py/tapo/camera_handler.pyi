from tapo.debug_ext import DebugExt
from tapo.device_handler_ext import DeviceHandlerExt
from tapo.responses import DeviceInfoCameraResult, RtspStreamUrl, Snapshot

class CameraHandler(DeviceHandlerExt, DebugExt):
    """Handler for Tapo cameras without PTZ, such as the
    [C120](https://www.tapo.com/en/search/?q=C120).
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def get_device_info(self) -> DeviceInfoCameraResult:
        """Returns *device info* as `DeviceInfoCameraResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `CameraHandler.get_device_info_json`.

        Returns:
            DeviceInfoCameraResult: Device info of Tapo cameras (C120, C210, C220, C225, C325WB, C520WS, TC40, TC70).
        """

    async def device_reboot(self) -> None:
        """*Reboots* the device.

        The device reboots right away. Subsequent commands may fail if sent during
        the reboot process or before the device reconnects to the network.
        """

    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
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
            RtspStreamUrl: The HD, SD and MJPEG RTSP stream URLs.
        """

    async def get_snapshot(self, username: str, password: str) -> Snapshot:
        """Captures a JPEG snapshot from the camera's dedicated MJPEG stream.

        The output resolution is fixed by the camera's jpegStream profile
        (approximately 640x360 on current models) and is not user-configurable.
        Typical latency is well under a second on a local network: each frame
        arrives already JPEG-encoded, so no decoder is involved.

        The credentials are the **camera account** credentials set in the Tapo app
        (Camera Settings > Advanced Settings > Camera Account), not the TP-Link cloud
        account credentials. They are the same credentials accepted by `get_rtsp_stream_url`.

        Args:
            username (str): The camera account username.
            password (str): The camera account password.

        Returns:
            Snapshot: The captured JPEG frame and its MIME content type.

        Raises:
            Exception: If the camera rejects the credentials, if it has no MJPEG stream,
                or if the snapshot doesn't finish within the `timeout_s` passed to `ApiClient`.
        """
