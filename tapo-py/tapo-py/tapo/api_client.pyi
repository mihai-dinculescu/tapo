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

from .generic_device_handler import GenericDeviceHandler
from .light_handler import LightHandler
from .color_light_handler import ColorLightHandler
from .plug_handler import PlugHandler
from .plug_energy_monitoring_handler import PlugEnergyMonitoringHandler

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
