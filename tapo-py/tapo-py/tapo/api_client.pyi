"""Tapo API Client.

Tested with light bulbs (L510, L520, L530, L535, L610, L630), light strips (L900, L920, L930), plugs (P100, P105, P110, P110M, P115),
power strips (P300, P304M, P306, P316M), hubs (H100), switches (S200B) and sensors (KE100, T100, T110, T300, T310, T315).

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

from .color_light_handler import ColorLightHandler
from .device_discovery import DeviceDiscovery
from .generic_device_handler import GenericDeviceHandler
from .hub_handler import HubHandler
from .light_handler import LightHandler
from .plug_energy_monitoring_handler import PlugEnergyMonitoringHandler
from .plug_handler import PlugHandler
from .power_strip_energy_monitoring_handler import PowerStripEnergyMonitoringHandler
from .power_strip_handler import PowerStripHandler
from .rgb_light_strip_handler import RgbLightStripHandler
from .rgbic_light_strip_handler import RgbicLightStripHandler

class ApiClient:
    """Tapo API Client.

    Tested with light bulbs (L510, L520, L530, L535, L610, L630), light strips (L900, L920, L930), plugs (P100, P105, P110, P110M, P115),
    power strips (P300, P304M, P306, P316M), hubs (H100), switches (S200B) and sensors (KE100, T100, T110, T300, T310, T315).

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

    def __init__(self, tapo_username: str, tapo_password: str, timeout_s: int = 30) -> None:
        """Returns a new instance of `ApiClient`.

        Args:
            tapo_username (str): The Tapo username.
            tapo_password (str): The Tapo password.
            timeout_s (int): The connection timeout in seconds. The default value is 30 seconds.

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

    async def discover_devices(self, target: str, timeout_s: int = 10) -> DeviceDiscovery:
        """Discovers one or more devices located at a specified unicast or broadcast IP address.

        Args:
            target (str): The IP address at which the discovery will take place.
                This address can be either a unicast (e.g. `192.168.1.10`) or a
                broadcast address (e.g. `192.168.1.255`, `255.255.255.255`, etc.).
            timeout_s (int): The maximum time to wait for a response from the device(s) in seconds.
                Must be between `1` and `60`.

        Returns:
            AsyncIterator[MaybeDiscoveryResult]: An asynchronous iterator that yields `MaybeDiscoveryResult` objects.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")

            async for device in client.discover_devices("192.168.1.255"):
                try:
                    device = discovery_result.get()
                    match device:
                        case DiscoveryResult.PlugEnergyMonitoring(device_info):
                            print(
                                f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                            )
                        # ...
                except Exception as e:
                    print(f"Error discovering device: {e}")
            ```
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
            LightHandler: Handler for the [L510](https://www.tapo.com/en/search/?q=L510),
            [L520](https://www.tapo.com/en/search/?q=L520) and [L610](https://www.tapo.com/en/search/?q=L610) devices.

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
            LightHandler: Handler for the [L510](https://www.tapo.com/en/search/?q=L510),
            [L520](https://www.tapo.com/en/search/?q=L520) and [L610](https://www.tapo.com/en/search/?q=L610) devices.

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
            ColorLightHandler: Handler for the [L530](https://www.tapo.com/en/search/?q=L530),
            [L535](https://www.tapo.com/en/search/?q=L535) and [L630](https://www.tapo.com/en/search/?q=L630) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l530("192.168.1.100")

            await device.on()
            ```
        """

    async def l535(self, ip_address: str) -> ColorLightHandler:
        """Specializes the given `ApiClient` into an authenticated `ColorLightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            ColorLightHandler: Handler for the [L530](https://www.tapo.com/en/search/?q=L530),
            [L535](https://www.tapo.com/en/search/?q=L535) and [L630](https://www.tapo.com/en/search/?q=L630) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l535("192.168.1.100")

            await device.on()
            ```
        """

    async def l610(self, ip_address: str) -> LightHandler:
        """Specializes the given `ApiClient` into an authenticated `LightHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            LightHandler: Handler for the [L510](https://www.tapo.com/en/search/?q=L510),
            [L520](https://www.tapo.com/en/search/?q=L520) and [L610](https://www.tapo.com/en/search/?q=L610) devices.

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
            ColorLightHandler: Handler for the [L530](https://www.tapo.com/en/search/?q=L530),
            [L630](https://www.tapo.com/en/search/?q=L630) and [L900](https://www.tapo.com/en/search/?q=L900) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l630("192.168.1.100")

            await device.on()
            ```
        """

    async def l900(self, ip_address: str) -> RgbLightStripHandler:
        """Specializes the given `ApiClient` into an authenticated `RgbLightStripHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            RgbLightStripHandler: Handler for the [L900](https://www.tapo.com/en/search/?q=L900) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l900("192.168.1.100")

            await device.on()
            ```
        """

    async def l920(self, ip_address: str) -> RgbicLightStripHandler:
        """Specializes the given `ApiClient` into an authenticated `RgbicLightStripHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            RgbicLightStripHandler: Handler for the [L920](https://www.tapo.com/en/search/?q=L920) and
            [L930](https://www.tapo.com/en/search/?q=L930) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l920("192.168.1.100")

            await device.on()
            ```
        """

    async def l930(self, ip_address: str) -> RgbicLightStripHandler:
        """Specializes the given `ApiClient` into an authenticated `RgbicLightStripHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            RgbicLightStripHandler: Handler for the [L920](https://www.tapo.com/en/search/?q=L920) and
            [L930](https://www.tapo.com/en/search/?q=L930) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.l930("192.168.1.100")

            await device.on()
            ```
        """

    async def p100(self, ip_address: str) -> PlugHandler:
        """Specializes the given `ApiClient` into an authenticated `PlugHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PlugHandler: Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
            [P105](https://www.tapo.com/en/search/?q=P105) devices.

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
            PlugHandler: Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
            [P105](https://www.tapo.com/en/search/?q=P105) devices.

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
            PlugEnergyMonitoringHandler: Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
            [P110M](https://www.tapo.com/en/search/?q=P110M) and
            [P115](https://www.tapo.com/en/search/?q=P115) devices.

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
            PlugEnergyMonitoringHandler: Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
            [P110M](https://www.tapo.com/en/search/?q=P110M) and
            [P115](https://www.tapo.com/en/search/?q=P115) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            device = await client.p115("192.168.1.100")

            await device.on()
            ```
        """

    async def p300(self, ip_address: str) -> PowerStripHandler:
        """Specializes the given `ApiClient` into an authenticated `PowerStripHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PowerStripHandler: Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
            [P306](https://www.tp-link.com/us/search/?q=P306) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            power_strip = await client.p300("192.168.1.100")

            child_device_list = await power_strip.get_child_device_list()
            print(f"Child device list: {child_device_list.to_dict()}")
            ```
        """

    async def p304(self, ip_address: str) -> PowerStripEnergyMonitoringHandler:
        """Specializes the given `ApiClient` into an authenticated `PowerStripEnergyMonitoringHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PowerStripEnergyMonitoringHandler: Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
            [P316M](https://www.tp-link.com/us/search/?q=P316M) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            power_strip = await client.p304("192.168.1.100")

            child_device_list = await power_strip.get_child_device_list()
            print(f"Child device list: {child_device_list.to_dict()}")
            ```
        """

    async def p306(self, ip_address: str) -> PowerStripHandler:
        """Specializes the given `ApiClient` into an authenticated `PowerStripHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PowerStripHandler: Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
            [P306](https://www.tp-link.com/us/search/?q=P306) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            power_strip = await client.p306("192.168.1.100")

            child_device_list = await power_strip.get_child_device_list()
            print(f"Child device list: {child_device_list.to_dict()}")
            ```
        """

    async def p316(self, ip_address: str) -> PowerStripHandler:
        """Specializes the given `ApiClient` into an authenticated `PowerStripHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            PowerStripEnergyMonitoringHandler: Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
            [P316M](https://www.tp-link.com/us/search/?q=P316M) devices.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            power_strip = await client.p316("192.168.1.100")

            child_device_list = await power_strip.get_child_device_list()
            print(f"Child device list: {child_device_list.to_dict()}")
            ```
        """

    async def h100(self, ip_address: str) -> HubHandler:
        """Specializes the given `ApiClient` into an authenticated `HubHandler`.

        Args:
            ip_address (str): The IP address of the device

        Returns:
            HubHandler: Handler for the [H100](https://www.tapo.com/en/search/?q=H100) hubs.

        Example:
            ```python
            client = ApiClient("tapo-username@example.com", "tapo-password")
            hub = await client.h100("192.168.1.100")

            child_device_list = await hub.get_child_device_list()
            print(f"Child device list: {child_device_list.to_dict()}")
            ```
        """
