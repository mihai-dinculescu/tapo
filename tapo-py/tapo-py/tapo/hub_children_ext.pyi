from typing import List, Optional, Protocol

from tapo import (
    KE100Handler,
    S200Handler,
    S210Handler,
    T100Handler,
    T110Handler,
    T300Handler,
    T31XHandler,
)
from tapo.responses import ChildDeviceComponentList

class HubChildrenExt(Protocol):
    """Extension class for the hub's child devices: their list as json, their
    component list and the handlers for the child devices that every hub model
    supports."""

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
        """

    async def ke100_unchecked(self, device_id: str) -> KE100Handler:
        """Returns a `KE100Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `get_child_device_list` call) to avoid
        the extra validation round-trip performed by `ke100`.
        """

    async def s200_unchecked(self, device_id: str) -> S200Handler:
        """Returns a `S200Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `get_child_device_list` call) to avoid
        the extra validation round-trip performed by `s200`.
        """

    async def s210_unchecked(self, device_id: str) -> S210Handler:
        """Returns a `S210Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `get_child_device_list` call) to avoid
        the extra validation round-trip performed by `s210`.
        """

    async def t100_unchecked(self, device_id: str) -> T100Handler:
        """Returns a `T100Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `get_child_device_list` call) to avoid
        the extra validation round-trip performed by `t100`.
        """

    async def t110_unchecked(self, device_id: str) -> T110Handler:
        """Returns a `T110Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `get_child_device_list` call) to avoid
        the extra validation round-trip performed by `t110`.
        """

    async def t300_unchecked(self, device_id: str) -> T300Handler:
        """Returns a `T300Handler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `get_child_device_list` call) to avoid
        the extra validation round-trip performed by `t300`.
        """

    async def t31x_unchecked(self, device_id: str) -> T31XHandler:
        """Returns a `T31XHandler` for the given `device_id` without first listing the hub's
        children to verify the device exists or matches the requested model. The device id
        is trusted; if it is wrong or refers to a different model, subsequent operations on
        the returned handler will fail at request time. Use this when you already have a
        valid device id (e.g. from a prior `get_child_device_list` call) to avoid
        the extra validation round-trip performed by `t31x`.
        """
