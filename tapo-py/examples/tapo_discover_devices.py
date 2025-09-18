"""Discover devices on the local network Example"""

import asyncio
import os

from tapo import ApiClient, DiscoveryResult


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    target = os.getenv("TARGET", "192.168.1.255")
    timeout_s = int(os.getenv("TIMEOUT", 10))

    print(f"Discovering Tapo devices on target: {target} for {timeout_s} seconds...")

    api_client = ApiClient(tapo_username, tapo_password)
    discovery = await api_client.discover_devices(target, timeout_s)

    async for discovery_result in discovery:
        try:
            device = discovery_result.get()

            match device:
                case DiscoveryResult.GenericDevice(device_info, _handler):
                    print(
                        f"Found Unsupported Device '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.Light(device_info, _handler):
                    print(
                        f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.ColorLight(device_info, _handler):
                    print(
                        f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.RgbLightStrip(device_info, _handler):
                    print(
                        f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.RgbicLightStrip(device_info, _handler):
                    print(
                        f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.Plug(device_info, _handler):
                    print(
                        f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.PlugEnergyMonitoring(device_info, _handler):
                    print(
                        f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.PowerStrip(device_info, _handler):
                    print(
                        f"Found Power Strip of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.PowerStripEnergyMonitoring(device_info, _handler):
                    print(
                        f"Found Power Strip with Energy Monitoring of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
                case DiscoveryResult.Hub(device_info, _handler):
                    print(
                        f"Found '{device_info.nickname}' of model '{device_info.model}' at IP address '{device_info.ip}'."
                    )
        except Exception as e:
            print(f"Error discovering device: {e}")


if __name__ == "__main__":
    asyncio.run(main())
