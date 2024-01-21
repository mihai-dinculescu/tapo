"""Tapo API Client.

Tested with light bulbs (L510, L520, L610) and plugs (P100, P105, P110, P115).

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
from typing import Optional, List

class ApiClient:
    """Tapo API Client.

    Tested with light bulbs (L510, L520, L610) and plugs (P100, P105, P110, P115).

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
    async def generic_device(self, ip_address: str) -> GenericDeviceHandler:
        """Specializes the given `ApiClient` into an authenticated `GenericDeviceHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            GenericDeviceHandler: Handler for generic devices. It provides the
            functionality common to all Tapo [devices](https://www.tapo.com/en/).

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.generic_device("192.168.1.100")

            await device.on()
            ```
        """
    async def l510(self, ip_address: str) -> LightHandler:
        """Specializes the given `ApiClient` into an authenticated `LightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            LightHandler: Handler for the [L510](https://www.tapo.com/en/search/?q=L510), [L520](https://www.tapo.com/en/search/?q=L520) and [L610](https://www.tapo.com/en/search/?q=L610) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l510("192.168.1.100")

            await device.on()
            ```
        """
    async def l520(self, ip_address: str) -> LightHandler:
        """Specializes the given `ApiClient` into an authenticated `LightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            LightHandler: Handler for the [L510](https://www.tapo.com/en/search/?q=L510), [L520](https://www.tapo.com/en/search/?q=L520) and [L610](https://www.tapo.com/en/search/?q=L610) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l520("192.168.1.100")

            await device.on()
            ```
        """
    async def l530(self, ip_address: str) -> ColorLightHandler:
        """Specializes the given `ApiClient` into an authenticated `ColorLightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            ColorLightHandler: Handler for the [L530](https://www.tapo.com/en/search/?q=L530), [L630](https://www.tapo.com/en/search/?q=L630) and [L900](https://www.tapo.com/en/search/?q=L900) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l530("192.168.1.100")

            await device.on()
            ```
        """
    async def l610(self, ip_address: str) -> LightHandler:
        """Specializes the given `ApiClient` into an authenticated `LightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            LightHandler: Handler for the [L510](https://www.tapo.com/en/search/?q=L510), [L520](https://www.tapo.com/en/search/?q=L520) and [L610](https://www.tapo.com/en/search/?q=L610) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l610("192.168.1.100")

            await device.on()
            ```
        """
    async def l630(self, ip_address: str) -> ColorLightHandler:
        """Specializes the given `ApiClient` into an authenticated `ColorLightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            ColorLightHandler: Handler for the [L530](https://www.tapo.com/en/search/?q=L530), [L630](https://www.tapo.com/en/search/?q=L630) and [L900](https://www.tapo.com/en/search/?q=L900) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l630("192.168.1.100")

            await device.on()
            ```
        """
    async def l900(self, ip_address: str) -> ColorLightHandler:
        """Specializes the given `ApiClient` into an authenticated `ColorLightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            ColorLightHandler: Handler for the [L530](https://www.tapo.com/en/search/?q=L530), [L630](https://www.tapo.com/en/search/?q=L630) and [L900](https://www.tapo.com/en/search/?q=L900) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l900("192.168.1.100")

            await device.on()
            ```
        """
    async def p100(self, ip_address: str) -> PlugHandler:
        """Specializes the given `ApiClient` into an authenticated `PlugHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PlugHandler: Handler for the [P100](https://www.tapo.com/en/search/?q=P100) & [P105](https://www.tapo.com/en/search/?q=P105) devices.

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
            PlugHandler: Handler for the [P100](https://www.tapo.com/en/search/?q=P100) & [P105](https://www.tapo.com/en/search/?q=P105) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p105("192.168.1.100")

            await device.on()
            ```
        """
    async def p110(self, ip_address: str) -> PlugEnergyMonitoringHandler:
        """Specializes the given `ApiClient` into an authenticated `PlugEnergyMonitoringHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PlugEnergyMonitoringHandler: Handler for the [P110](https://www.tapo.com/en/search/?q=P110) & [P115](https://www.tapo.com/en/search/?q=P115) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p110("192.168.1.100")

            await device.on()
            ```
        """
    async def p115(self, ip_address: str) -> PlugEnergyMonitoringHandler:
        """Specializes the given `ApiClient` into an authenticated `PlugEnergyMonitoringHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PlugEnergyMonitoringHandler: Handler for the [P110](https://www.tapo.com/en/search/?q=P110) & [P115](https://www.tapo.com/en/search/?q=P115) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p115("192.168.1.100")

            await device.on()
            ```
        """

class GenericDeviceHandler:
    """Handler for generic devices. It provides the functionality common to
    all Tapo [devices](https://www.tapo.com/en/).
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
    async def get_device_info(self) -> DeviceInfoGenericResult:
        """Returns *device info* as `DeviceInfoGenericResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `GenericDeviceHandler.get_device_info_json`.

        Returns:
            DeviceInfoGenericResult: Device info of a Generic Tapo device.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

class LightHandler:
    """Handler for the [L510](https://www.tapo.com/en/search/?q=L510), [L520](https://www.tapo.com/en/search/?q=L520) and [L610](https://www.tapo.com/en/search/?q=L610) devices."""

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
    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """
    async def get_device_info(self) -> DeviceInfoLightResult:
        """Returns *device info* as `DeviceInfoLightResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `LightHandler.get_device_info_json`.

        Returns:
            DeviceInfoLightResult: Device info of Tapo L510, L520 and L610.
            Superset of `GenericDeviceInfoResult`.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """
    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time usage.
        """
    async def set_brightness(self, brightness: int) -> None:
        """Sets the *brightness* and turns *on* the device.

        Args:
            brightness (int): between 1 and 100
        """

class ColorLightHandler:
    """Handler for the [L530](https://www.tapo.com/en/search/?q=L530), [L630](https://www.tapo.com/en/search/?q=L630) and [L900](https://www.tapo.com/en/search/?q=L900) devices."""

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
    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """
    async def get_device_info(self) -> DeviceInfoColorLightResult:
        """Returns *device info* as `DeviceInfoColorLightResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `ColorLightHandler.get_device_info_json`.

        Returns:
            DeviceInfoColorLightResult: Device info of Tapo L530, L630 and L900.
            Superset of `GenericDeviceInfoResult`.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """
    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time usage.
        """
    async def set_brightness(self, brightness: int) -> None:
        """Sets the *brightness* and turns *on* the device.

        Args:
            brightness (int): between 1 and 100
        """
    async def set_color(self, color: Color) -> None:
        """Sets the *color* and turns *on* the device.

        Args:
            color (Color): one of `tapo.Color` as defined in the Google Home app.
        """
    async def set_hue_saturation(self, hue: int, saturation: int) -> None:
        """Sets the *hue*, *saturation* and turns *on* the device.

        Args:
            hue (int): between 1 and 360
            saturation (int): between 1 and 100
        """
    async def set_color_temperature(self, color_temperature: int) -> None:
        """Sets the *color temperature* and turns *on* the device.

        Args:
            color_temperature (int): between 2500 and 6500
        """

class PlugHandler:
    """Handler for the [P100](https://www.tapo.com/en/search/?q=P100) & [P105](https://www.tapo.com/en/search/?q=P105) devices."""

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
    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """
    async def get_device_info(self) -> DeviceInfoPlugResult:
        """Returns *device info* as `DeviceInfoPlugResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PlugHandler.get_device_info_json`.

        Returns:
            DeviceInfoPlugResult: Device info of Tapo P100, P105, P110 and P115.
            Superset of `GenericDeviceInfoResult`.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """
    async def get_device_usage(self) -> DeviceUsageResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageResult: Contains the time usage.
        """

class PlugEnergyMonitoringHandler:
    """Handler for the [P110](https://www.tapo.com/en/search/?q=P110) & [P115](https://www.tapo.com/en/search/?q=P115) devices."""

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
    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """
    async def get_device_info(self) -> DeviceInfoPlugResult:
        """Returns *device info* as `DeviceInfoPlugResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PlugEnergyMonitoringHandler.get_device_info_json`.

        Returns:
            DeviceInfoPlugResult: Device info of Tapo P100, P105, P110 and P115.
            Superset of `GenericDeviceInfoResult`.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """
    async def get_device_usage(self) -> DeviceUsageEnergyMonitoringResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageEnergyMonitoringResult:
            Contains the time usage, the power consumption, and the energy savings of the device.
        """
    async def get_current_power(self) -> CurrentPowerResult:
        """Returns *current power* as `CurrentPowerResult`.

        Returns:
            CurrentPowerResult: Contains the current power reading of the device.
        """
    async def get_energy_usage(self) -> EnergyUsageResult:
        """Returns *energy usage* as `EnergyUsageResult`.

        Returns:
            EnergyUsageResult:
            Contains local time, current power and the energy usage and runtime for today and for the current month.
        """
    async def get_energy_data(
        self,
        interval: EnergyDataInterval,
        start_date: datetime,
        end_date: datetime = None,
    ) -> EnergyDataResult:
        """Returns *energy data* as `EnergyDataResult`.

        Returns:
            EnergyDataResult: Energy data for the requested `EnergyDataInterval`.
        """

class DefaultStateType(StrEnum):
    """The type of the default state."""

    Custom = "custom"
    LastStates = "last_states"

class DefaultPowerType(StrEnum):
    """The type of the default power state."""

    AlwaysOn = "always_on"
    LastStates = "last_states"

class DefaultBrightnessState:
    """Default brightness state."""

    type: DefaultStateType
    value: int

class DeviceInfoGenericResult:
    """Device info of a Generic Tapo device."""

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
    device_on: Optional[bool]
    on_time: Optional[int]
    """The time in seconds this device has been ON since the last state change (ON/OFF)."""
    overheated: bool
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DeviceInfoLightResult:
    """Device info of Tapo L510, L520 and L610. Superset of `GenericDeviceInfoResult`."""

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

    # Unique to this device
    brightness: int
    dynamic_light_effect_enable: bool
    dynamic_light_effect_id: Optional[str]
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: int
    default_states: DefaultLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DefaultLightState:
    """Light Default State."""

    brightness: DefaultBrightnessState
    re_power_type: Optional[DefaultPowerType]

class DeviceInfoColorLightResult:
    """Device info of Tapo L530, L630 and L900. Superset of `GenericDeviceInfoResult`."""

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

    # Unique to this device
    brightness: int
    dynamic_light_effect_enable: bool
    dynamic_light_effect_id: Optional[str]
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: int
    default_states: DefaultColorLightState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DefaultColorLightState:
    """Color Light Default State."""

    type: DefaultStateType
    state: ColorLightState

class ColorLightState:
    """Color Light State."""

    brightness: int
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: int

class DeviceInfoPlugResult:
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

    # Unique to this device
    default_states: PlugDefaultState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class PlugDefaultState:
    """Plug Default State."""

    type: DefaultStateType
    state: PlugState

class PlugState:
    """Plug State."""

    on: Optional[bool]

class UsageByPeriodResult:
    """Usage by period result for today, the past 7 days, and the past 30 days."""

    today: int
    """Today."""
    past7: int
    """Past 7 days."""
    past30: int
    """Past 30 days."""

class DeviceUsageResult:
    """Contains the time in use, the power consumption, and the energy savings of the device."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""
    power_usage: UsageByPeriodResult
    """Power usage in watt-hour (Wh)."""
    saved_power: UsageByPeriodResult
    """Saved power in watt-hour (Wh)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DeviceUsageEnergyMonitoringResult:
    """Contains the time in use, the power consumption, and the energy savings of the device."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""
    power_usage: UsageByPeriodResult
    """Power usage in watt-hour (Wh)."""
    saved_power: UsageByPeriodResult
    """Saved power in watt-hour (Wh)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class CurrentPowerResult:
    """Contains the current power reading of the device."""

    current_power: int
    """Current power in watts (W)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

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
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class EnergyDataResult:
    """Energy data for the requested `EnergyDataInterval`."""

    local_time: datetime
    """Local time of the device."""

    data: List[int]
    """Energy data for the given `interval` in watts (W)."""

    start_timestamp: int
    """Interval start timestamp in milliseconds."""

    end_timestamp: int
    """Interval end timestamp in milliseconds."""

    interval: int
    """Interval in minutes."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class EnergyDataInterval(StrEnum):
    """Energy data interval."""

    Hourly = "Hourly"
    """Hourly interval. `start_date` and `end_date` are an inclusive interval
    that must not be greater than 8 days.
    """

    Daily = "Daily"
    """Daily interval. `start_date` must be the first day of a quarter."""

    Monthly = "Monthly"
    """Monthly interval. `start_date` must be the first day of a year."""

class Color(StrEnum):
    """List of preset colors as defined in the Google Home app."""

    CoolWhite = "CoolWhite"
    Daylight = "Daylight"
    Ivory = "Ivory"
    WarmWhite = "WarmWhite"
    Incandescent = "Incandescent"
    Candlelight = "Candlelight"
    Snow = "Snow"
    GhostWhite = "GhostWhite"
    AliceBlue = "AliceBlue"
    LightGoldenrod = "LightGoldenrod"
    LemonChiffon = "LemonChiffon"
    AntiqueWhite = "AntiqueWhite"
    Gold = "Gold"
    Peru = "Peru"
    Chocolate = "Chocolate"
    SandyBrown = "SandyBrown"
    Coral = "Coral"
    Pumpkin = "Pumpkin"
    Tomato = "Tomato"
    Vermilion = "Vermilion"
    OrangeRed = "OrangeRed"
    Pink = "Pink"
    Crimson = "Crimson"
    DarkRed = "DarkRed"
    HotPink = "HotPink"
    Smitten = "Smitten"
    MediumPurple = "MediumPurple"
    BlueViolet = "BlueViolet"
    Indigo = "Indigo"
    LightSkyBlue = "LightSkyBlue"
    CornflowerBlue = "CornflowerBlue"
    Ultramarine = "Ultramarine"
    DeepSkyBlue = "DeepSkyBlue"
    Azure = "Azure"
    NavyBlue = "NavyBlue"
    LightTurquoise = "LightTurquoise"
    Aquamarine = "Aquamarine"
    Turquoise = "Turquoise"
    LightGreen = "LightGreen"
    Lime = "Lime"
    ForestGreen = "ForestGreen"
