"""Tapo API Client.

Tested with light bulbs (L510, L530, L610, L630), light strips (L900, L920, L930),
plugs (P100, P105, P110, P115), hubs (H100), switches (S200B) and sensors (T100, T110, T310, T315).

Example:
    ```python
    import asyncio
    from tapo import ApiClient


    async def main():
        client = ApiClient("tapo-username@example.com", "tapo-password")
        device = await client.l530("192.168.1.100")

        await device.on()

    if __name__ == "__main__":
        asyncio.run(main())
    ```

See [more examples](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-py/examples).
"""


from datetime import datetime
from enum import StrEnum
from typing import Optional


class ApiClient:
    """Tapo API Client.

    Tested with light bulbs (L510, L530, L610, L630), light strips (L900, L920, L930),
    plugs (P100, P105, P110, P115), hubs (H100), switches (S200B) and sensors (T100, T110, T310, T315).

    Example:
        ```python
        import asyncio
        from tapo import ApiClient


        async def main():
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l530("192.168.1.100")

            await device.on()

        if __name__ == "__main__":
            asyncio.run(main())
        ```

    See [more examples](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-py/examples).
    """

    def __init__(self, tapo_username: str, tapo_password: str) -> None:
        """Returns a new instance of `ApiClient`.

        Args:
            tapo_username (str): The Tapo username
            tapo_password (str): The Tapo password

        Returns:
            ApiClient: Tapo API Client.

        Example:
            ```python
            import asyncio
            from tapo import ApiClient


            async def main():
                client = ApiClient("tapo-username@example.com", "tapo-password")
                device = await client.l530("192.168.1.100")

                await device.on()

            if __name__ == "__main__":
                asyncio.run(main())
            ```

        See [more examples](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-py/examples).
        """

    async def p100(self, ip_address: str) -> PlugHandler:
        """Specializes the given `ApiClient` into an authenticated `PlugHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PlugHandler: Handler for the [P100](https://www.tapo.com/en/search/?q=P100) 
            & [P105](https://www.tapo.com/en/search/?q=P105) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p100("192.168.1.100")

            await device.on()
            ```
        """

    async def p105(self, ip_address: str) -> PlugHandler:
        """Specializes the given `ApiClient` into an authenticated `PlugHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PlugHandler: Handler for the [P100](https://www.tapo.com/en/search/?q=P100) 
            & [P105](https://www.tapo.com/en/search/?q=P105) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p105("192.168.1.100")

            await device.on()
            ```
        """

    async def p110(self, ip_address: str) -> EnergyMonitoringPlugHandler:
        """Specializes the given `ApiClient` into an authenticated `EnergyMonitoringPlugHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            EnergyMonitoringPlugHandler: Handler for the [P110](https://www.tapo.com/en/search/?q=P110) 
            & [P115](https://www.tapo.com/en/search/?q=P115) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p110("192.168.1.100")

            await device.on()
            ```
        """

    async def p115(self, ip_address: str) -> EnergyMonitoringPlugHandler:
        """Specializes the given `ApiClient` into an authenticated `EnergyMonitoringPlugHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            EnergyMonitoringPlugHandler: Handler for the [P110](https://www.tapo.com/en/search/?q=P110) 
            & [P115](https://www.tapo.com/en/search/?q=P115) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p115("192.168.1.100")

            await device.on()
            ```
        """


class PlugHandler:
    """Handler for the [P100](https://www.tapo.com/en/search/?q=P100) 
    & [P105](https://www.tapo.com/en/search/?q=P105) devices.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""

    async def on(self) -> None:
        """Turns *on* the device."""

    async def off(self) -> None:
        """Turns *off* the device."""

    async def get_device_info(self) -> PlugDeviceInfoResult:
        """Returns *device info* as `PlugDeviceInfoResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present, 
        try `PlugDeviceInfoResult.get_device_info_json`.

        Returns:
            PlugDeviceInfoResult: Device info of Tapo P100, P105, P110 and P115.
            Superset of `GenericDeviceInfoResult`.
        """

    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time in use, the power consumption, and the energy savings of the device.
        """


class EnergyMonitoringPlugHandler:
    """Handler for the [P110](https://www.tapo.com/en/search/?q=P110) 
    & [P115](https://www.tapo.com/en/search/?q=P115) devices.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""

    async def on(self) -> None:
        """Turns *on* the device."""

    async def off(self) -> None:
        """Turns *off* the device."""

    async def get_device_info(self) -> PlugDeviceInfoResult:
        """Returns *device info* as `PlugDeviceInfoResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present, 
        try `PlugDeviceInfoResult.get_device_info_json`.

        Returns:
            PlugDeviceInfoResult: Device info of Tapo P100, P105, P110 and P115.
            Superset of `GenericDeviceInfoResult`.
        """

    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time in use, the power consumption, and the energy savings of the device.
        """

    async def get_energy_usage(self) -> EnergyUsageResult:
        """Returns *energy usage* as `EnergyUsageResult`.

        Returns:
            EnergyUsageResult: Contains local time, current power and the energy usage and runtime for today and for the current month.
        """


class PlugDeviceInfoResult:
    """Device info of Tapo P100, P105, P110 and P115. Superset of `GenericDeviceInfoResult`."""

    device_id: str
    type: str
    model: str
    hw_id: str
    hw_ver: str
    fw_id: str
    fw_ver: str
    oem_id: str
    mac: str
    ip: str
    ssid: str
    signal_level: int
    rssi: int
    specs: str
    lang: str
    device_on: bool
    on_time: int
    """The time in seconds this device has been ON since the last state change (ON/OFF)."""
    overheated: bool
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]
    default_states: PlugDefaultState

    def to_dict(self) -> dict:
        """Get all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """


class PlugDefaultState:
    """Plug Default State."""

    type: DefaultStateType
    state: PlugState


class DefaultStateType(StrEnum):
    """The type of the default state."""

    Custom = "custom"
    LastStates = "last_states"


class PlugState:
    """Plug State."""

    on: Optional[bool]


class DeviceUsageResult:
    """Contains the time in use, the power consumption, and the energy savings of the device."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""
    power_usage: UsageByPeriodResult
    """Power usage in watt-hour (Wh)."""
    saved_power: UsageByPeriodResult
    """Saved power in watt-hour (Wh)."""

    def to_dict(self) -> dict:
        """Get all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """


class UsageByPeriodResult:
    """Usage by period result for today, the past 7 days, and the past 30 days."""

    today: int
    """Today."""
    past7: int
    """Past 7 days."""
    past30: int
    """Past 30 days."""


class EnergyUsageResult:
    """Contains local time, current power and the energy usage and runtime for today and for the current month."""

    local_time: datetime
    """Local time of the device."""
    current_power: int
    """Current power in milliwatts (mW)."""
    today_runtime: int
    """Today runtime in minutes."""
    today_energy: int
    """Today energy usage in watts (W)."""
    month_runtime: int
    """Current month runtime in minutes."""
    month_energy: int
    """Current month energy usage in watts (W)."""

    def to_dict(self) -> dict:
        """Get all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
