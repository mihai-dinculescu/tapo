from datetime import datetime
from os import PathLike
from typing import List, Optional, Union
from zoneinfo import ZoneInfo

from tapo.debug_ext import DebugExt
from tapo.device_handler_ext import DeviceHandlerExt
from tapo.hub_children_ext import HubChildrenExt
from tapo.responses import (
    DeviceInfoCameraHubResult,
    GeneralDeviceHubResult,
    KE100Result,
    OtherResult,
    RecordingDateHubResult,
    RecordingDownloadResult,
    RecordingHubResult,
    S200Result,
    S210Result,
    T100Result,
    T110Result,
    T300Result,
    T31XResult,
    TimezoneResult,
)

class CameraHubHandler(DeviceHandlerExt, HubChildrenExt, DebugExt):
    """Handler for camera hubs, such as the
    [H200](https://www.tapo.com/en/search/?q=H200) and
    [H500](https://www.tapo.com/en/search/?q=H500).

    Example:
        ```python
        # Connect to the hub
        client = ApiClient("tapo-username@example.com", "tapo-password")
        hub = await client.h200("192.168.1.100")

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

    async def get_device_info(self) -> DeviceInfoCameraHubResult:
        """Returns *device info* as `DeviceInfoCameraHubResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `CameraHubHandler.get_device_info_json`.

        Returns:
            DeviceInfoCameraHubResult: Device info of Tapo H200 and H500.
        """

    async def get_general_device_list(self) -> List[GeneralDeviceHubResult]:
        """Returns *general device list* as `List[GeneralDeviceHubResult]`.
        These are the standalone Wi-Fi cameras paired to the hub.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `CameraHubHandler.get_general_device_list_json`.

        Returns:
            List[GeneralDeviceHubResult]: The cameras paired to the hub.
        """

    async def get_general_device_list_json(self) -> dict:
        """Returns *general device list* as json.
        It contains all the properties returned from the Tapo API for the
        standalone Wi-Fi cameras paired to the hub.

        Returns:
            dict: General device list as a dictionary.
        """

    async def get_timezone(self) -> TimezoneResult:
        """Returns the hub's *timezone* as `TimezoneResult`.

        Returns:
            TimezoneResult: The timezone configured on the hub.

        Raises:
            Exception: If the hub reports a timezone name that is not in the IANA database.
        """

    async def get_recording_dates(
        self,
        child_device_id: str,
        start_time: datetime,
        end_time: datetime,
        timezone: Optional[ZoneInfo] = None,
    ) -> List[RecordingDateHubResult]:
        """Returns the days that have recordings stored on the hub for the given
        camera, as `List[RecordingDateHubResult]`.
        Each day comes with the range of time it covers on the hub, ready to
        pass to `CameraHubHandler.get_recordings`.

        The days are searched in the hub's timezone, so they line up with the
        Tapo app wherever this code runs. Each day that the range touches is
        searched whole: a range that starts at 00:30 covers all of that day,
        and a returned day can reach past the range at either end.

        Pass the `TimezoneResult.timezone` attribute from
        `CameraHubHandler.get_timezone` to read it once for several calls.
        Without it, each call reads it, which takes one extra request.

        Args:
            child_device_id (str): The `device_id` of a camera returned by
                `CameraHubHandler.get_general_device_list`.
            start_time (datetime): The start of the search range. Must be timezone-aware.
            end_time (datetime): The end of the search range. Must be timezone-aware.
            timezone (Optional[ZoneInfo]): The hub's timezone. Without it, it is read from the hub.

        Returns:
            List[RecordingDateHubResult]: The days with recordings, in the order the hub reports them.

        Raises:
            TypeError: If `start_time` or `end_time` is a naive datetime.
            Exception: If `end_time` is before `start_time`, or if the hub reports
                a timezone name that is not in the IANA database.
        """

    async def get_recordings(
        self,
        child_device_id: str,
        start_time: datetime,
        end_time: datetime,
    ) -> List[RecordingHubResult]:
        """Returns the recordings stored on the hub for the given camera, within
        the given time range, as `List[RecordingHubResult]`.
        Like the Tapo app, all pages are fetched, up to 12,300 recordings.

        Args:
            child_device_id (str): The `device_id` of a camera returned by
                `CameraHubHandler.get_general_device_list`.
            start_time (datetime): The start of the search range. Must be timezone-aware.
            end_time (datetime): The end of the search range. Must be timezone-aware.

        Returns:
            List[RecordingHubResult]: The recordings, in the order the hub reports them.

        Raises:
            TypeError: If `start_time` or `end_time` is a naive datetime.
            Exception: If `end_time` is before `start_time`, or if either is before 1970.
        """

    async def download_recording(
        self,
        child_device_id: str,
        start_time: datetime,
        end_time: datetime,
        path: Union[str, PathLike],
    ) -> RecordingDownloadResult:
        """Downloads a recording stored on the hub to a file, writing the media
        as it arrives.

        The hub streams the recording as encrypted MPEG-TS; the parts are
        decrypted with keys derived from the session's key exchange, so a file
        with a `.ts` extension is a playable clip. The hub ends the clip itself,
        with twice the clip's length on top of the client's timeout as a backstop.

        The media is written exactly as the hub sent it, with nothing added or
        re-muxed, so a player may log a one-off `TS discontinuity` warning for
        the stream's PAT and PMT when it opens the file. It is harmless: both
        tables are repeated throughout the stream and the clip plays in full.

        Args:
            child_device_id (str): The `device_id` of a camera returned by
                `CameraHubHandler.get_general_device_list`.
            start_time (datetime): The `start_time` of a recording returned by
                `CameraHubHandler.get_recordings`. Must be timezone-aware.
            end_time (datetime): The `end_time` of that recording. Must be timezone-aware.
            path (Union[str, PathLike]): The file to write the media to.
                It is created, or truncated if it already exists.

        Returns:
            RecordingDownloadResult: How much media was written and the playback time it covers.

        Raises:
            TypeError: If `start_time` or `end_time` is a naive datetime.
            Exception: If `end_time` is not after `start_time`, if either is
                before 1970, if the file cannot be created, if the hub rejects
                the request or sends no media, if it sends encrypted media that
                cannot be decrypted, if writing to the file fails, or if the
                hub closes the connection or the backstop runs out before the
                hub reports the end of the recording. In that last case the
                file holds only the part of the recording that arrived.

        Example:
            ```python
            from datetime import datetime, timedelta, timezone

            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            camera = (await hub.get_general_device_list())[0]

            end_time = datetime.now(timezone.utc)
            start_time = end_time - timedelta(days=1)
            recordings = await hub.get_recordings(camera.device_id, start_time, end_time)

            if recordings:
                recording = recordings[0]
                result = await hub.download_recording(
                    camera.device_id, recording.start_time, recording.end_time, "recording.ts"
                )
                print(f"Wrote {result.byte_count} bytes to recording.ts")
            ```
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
        try `CameraHubHandler.get_child_device_list_json`.
        Cameras paired to the hub are not included; use
        `CameraHubHandler.get_general_device_list` for those.

        Returns:
            List[KE100Result | S200Result | S210Result | T100Result | T110Result | T300Result | T31XResult | OtherResult]: The child devices paired to the hub.
        """
