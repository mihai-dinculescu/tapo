from typing import List, Optional, Union

from tapo import KE100Handler, S200BHandler, T100Handler, T110Handler, T300Handler, T31XHandler
from tapo.requests.play_alarm import AlarmDuration, AlarmRingtone, AlarmVolume
from tapo.responses import (
    DeviceInfoHubResult,
    KE100Result,
    S200BResult,
    T100Result,
    T110Result,
    T300Result,
    T31XResult,
)

class HubHandler:
    """Handler for the [H100](https://www.tapo.com/en/search/?q=H100) hubs."""

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""

    async def get_device_info(self) -> DeviceInfoHubResult:
        """Returns *device info* as `DeviceInfoHubResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `HubHandler.get_device_info_json`.

        Returns:
            DeviceInfoHubResult: Device info of Tapo H100.
            Superset of `GenericDeviceInfoResult`.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_child_device_list(
        self,
    ) -> List[
        Union[KE100Result, S200BResult, T100Result, T110Result, T300Result, T31XResult, None]
    ]:
        """Returns *child device list* as `List[KE100Result | S200BResult | T100Result | T110Result | T300Result | T31XResult | None]`.
        It is not guaranteed to contain all the properties returned from the Tapo API
        or to support all the possible devices connected to the hub.
        If the deserialization fails, or if a property that you care about it's not present,
        try `HubHandler.get_child_device_list_json`.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_child_device_list_json(self, start_index: int) -> dict:
        """Returns *child device list* as json.
        It contains all the properties returned from the Tapo API.

        Args:
            start_index (int): the index to start fetching the child device list.
            It should be `0` for the first page, `10` for the second, and so on.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_child_device_component_list_json(self) -> dict:
        """Returns *child device component list* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_supported_ringtone_list() -> List[str]:
        """Returns a list of ringtones (alarm types) supported by the hub.
        Used for debugging only.

        Returns:
            List[str]: List of the ringtones supported by the hub.
        """

    async def play_alarm(
        self,
        ringtone: AlarmRingtone,
        volume: AlarmVolume,
        duration: AlarmDuration,
        seconds: Optional[int] = None,
    ) -> None:
        """Start playing the hub alarm.

        Args:
            ringtone (AlarmRingtone): The ringtone of a H100 alarm.
            volume (AlarmVolume): The volume of the alarm.
            duration (AlarmDuration): Controls how long the alarm plays for.
            seconds (Optional[int]): Play the alarm a number of seconds. Required if `duration` is `AlarmDuration.Seconds`.
        """

    async def stop_alarm(self) -> None:
        """Stop playing the hub alarm, if it's currently playing."""

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
            hub = await client.h100("192.168.1.100")

            # Get a handler for the child device
            device = await hub.ke100(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def s200b(
        self, device_id: Optional[str] = None, nickname: Optional[str] = None
    ) -> S200BHandler:
        """Returns a `S200BHandler` for the device matching the provided `device_id` or `nickname`.

        Args:
            device_id (Optional[str]): The Device ID of the device
            nickname (Optional[str]): The Nickname of the device

        Returns:
            S200BHandler: Handler for the [S200B](https://www.tapo.com/en/search/?q=S200B) devices.

        Example:
            ```python
            # Connect to the hub
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h100("192.168.1.100")

            # Get a handler for the child device
            device = await hub.s200b(device_id="0000000000000000000000000000000000000000")

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
            hub = await client.h100("192.168.1.100")

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
            hub = await client.h100("192.168.1.100")

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
            hub = await client.h100("192.168.1.100")

            # Get a handler for the child device
            device = await hub.t300(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def t310(
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
            hub = await client.h100("192.168.1.100")

            # Get a handler for the child device
            device = await hub.t310(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """

    async def t315(
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
            hub = await client.h100("192.168.1.100")

            # Get a handler for the child device
            device = await hub.t315(device_id="0000000000000000000000000000000000000000")

            # Get the device info of the child device
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")
            ```
        """
