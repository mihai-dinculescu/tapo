from datetime import datetime
from os import PathLike
from typing import List, Optional, Union

from tapo import (
    KE100Handler,
    S200Handler,
    S210Handler,
    T100Handler,
    T110Handler,
    T300Handler,
    T31XHandler,
)
from tapo.debug_ext import DebugExt
from tapo.refresh_session_ext import RefreshSessionExt
from tapo.responses import (
    ChildDeviceComponentList,
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
    TimezoneHubResult,
)

class CameraHubHandler(RefreshSessionExt, DebugExt):
    """Handler for camera hubs, such as the
    [H200](https://www.tapo.com/en/search/?q=H200) and
    [H500](https://www.tapo.com/en/search/?q=H500).
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

    async def get_timezone(self) -> TimezoneHubResult:
        """Returns the hub's *timezone* as `TimezoneHubResult`.

        Returns:
            TimezoneHubResult: The timezone configured on the hub.

        Raises:
            Exception: If the hub reports a timezone name that is not in the IANA database.
        """

    async def get_recording_dates(
        self,
        child_device_id: str,
        start_time: datetime,
        end_time: datetime,
    ) -> List[RecordingDateHubResult]:
        """Returns the days that have recordings stored on the hub for the given
        camera, as `List[RecordingDateHubResult]`.
        Each day comes with the range of time it covers on the hub, ready to
        pass to `CameraHubHandler.get_recordings`.

        The hub searches its own calendar, so the search runs over the days
        that the given range touches in the hub's timezone, whole. A range that
        starts at 00:30 covers all of that day, and a returned day can reach
        past the range at either end.

        Reading the hub's timezone takes one extra request
        (see `CameraHubHandler.get_timezone`).

        Args:
            child_device_id (str): The `device_id` of a camera returned by
                `CameraHubHandler.get_general_device_list`.
            start_time (datetime): The start of the search range. Must be timezone-aware.
            end_time (datetime): The end of the search range. Must be timezone-aware.

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

    async def get_child_device_list_json(self, start_index: int) -> dict:
        """Returns *child device list* as json.
        It contains all the properties returned from the Tapo API.

        Args:
            start_index (int): the index to start fetching the child device list.
                It should be `0` for the first page, `10` for the second, and so on.

        Returns:
            dict: Child device list as a dictionary.
        """

    async def get_child_device_component_list(self) -> List[ChildDeviceComponentList]:
        """Returns *child device component list* as a list of `ChildDeviceComponentList`.
        This information is useful in debugging or when investigating new functionality to add.

        Returns:
            List[ChildDeviceComponentList]: The component list for each child device.
        """

    async def ke100(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> KE100Handler:
        """Returns a `KE100Handler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            KE100Handler: Handler for the [KE100](https://www.tp-link.com/en/search/?q=KE100) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            # Get a handler for the child device
            device = await hub.ke100(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def s200(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> S200Handler:
        """Returns a `S200Handler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            S200Handler: Handler for the [S200B](https://www.tapo.com/en/search/?q=S200B) and
            [S200D](https://www.tapo.com/en/search/?q=S200D) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            # Get a handler for the child device
            device = await hub.s200(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def s210(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> S210Handler:
        """Returns a `S210Handler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            S210Handler: Handler for the [S210](https://www.tapo.com/en/search/?q=S210) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            # Get a handler for the child device
            device = await hub.s210(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def t100(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> T100Handler:
        """Returns a `T100Handler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            T100Handler: Handler for the [T100](https://www.tapo.com/en/search/?q=T100) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            # Get a handler for the child device
            device = await hub.t100(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def t110(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> T110Handler:
        """Returns a `T110Handler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            T110Handler: Handler for the [T110](https://www.tapo.com/en/search/?q=T110) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            # Get a handler for the child device
            device = await hub.t110(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def t300(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> T300Handler:
        """Returns a `T300Handler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            T300Handler: Handler for the [T300](https://www.tapo.com/en/search/?q=T300) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            # Get a handler for the child device
            device = await hub.t300(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def t31x(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> T31XHandler:
        """Returns a `T31XHandler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            T31XHandler: Handler for the [T310](https://www.tapo.com/en/search/?q=T310)
            and [T315](https://www.tapo.com/en/search/?q=T315) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h200("192.168.1.100")

            # Get a handler for the child device
            device = await hub.t31x(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def ke100_unchecked(self, device_id: str) -> KE100Handler:
        """Returns a `KE100Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `CameraHubHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `CameraHubHandler.ke100`.
        """

    async def s200_unchecked(self, device_id: str) -> S200Handler:
        """Returns a `S200Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `CameraHubHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `CameraHubHandler.s200`.
        """

    async def s210_unchecked(self, device_id: str) -> S210Handler:
        """Returns a `S210Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `CameraHubHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `CameraHubHandler.s210`.
        """

    async def t100_unchecked(self, device_id: str) -> T100Handler:
        """Returns a `T100Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `CameraHubHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `CameraHubHandler.t100`.
        """

    async def t110_unchecked(self, device_id: str) -> T110Handler:
        """Returns a `T110Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `CameraHubHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `CameraHubHandler.t110`.
        """

    async def t300_unchecked(self, device_id: str) -> T300Handler:
        """Returns a `T300Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `CameraHubHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `CameraHubHandler.t300`.
        """

    async def t31x_unchecked(self, device_id: str) -> T31XHandler:
        """Returns a `T31XHandler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `CameraHubHandler.get_child_device_list` call) to avoid
        the extra validation round-trip performed by `CameraHubHandler.t31x`.
        """
