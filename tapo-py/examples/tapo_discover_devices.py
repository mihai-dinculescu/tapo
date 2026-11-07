"""Discover devices on the local network Example"""

import asyncio
import os

from tapo import ApiClient, DeviceType, DiscoveryResult

from common import require_env_vars


async def main():
    tapo_username, tapo_password = require_env_vars("TAPO_USERNAME", "TAPO_PASSWORD")
    target = os.getenv("TAPO_DISCOVERY_TARGET", "192.168.1.255")
    timeout_s = int(os.getenv("TAPO_DISCOVERY_TIMEOUT", 10))

    print(f"Discovering Tapo devices on target: {target} for {timeout_s} seconds...")

    api_client = ApiClient(tapo_username, tapo_password)
    discovery = await api_client.discover_devices(target, timeout_s)

    # NOTE: This example uses explicit pattern matching to demonstrate all
    # DiscoveryResult variants and the way in which they expose `device_info`
    # and `handler`. For a more concise approach, you can use the
    # accessor properties instead for a few common properties:
    #
    #   device_id = discovery_result.device_id
    #   nickname = discovery_result.nickname
    #   model = discovery_result.model
    #   address = discovery_result.address
    #   device_type = discovery_result.device_type

    async for discovery_result in discovery:
        try:
            discovery_result = discovery_result.get()

            match discovery_result:
                case DiscoveryResult.Light(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.Light))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.ColorLight(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.ColorLight))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.RgbLightStrip(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.RgbLightStrip))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.RgbicLightStrip(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.RgbicLightStrip))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.Plug(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.Plug))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.PlugEnergyMonitoring(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.PlugEnergyMonitoring))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.PowerStrip(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.PowerStrip))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.PowerStripEnergyMonitoring(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.PowerStripEnergyMonitoring))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.Hub(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.Hub))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.HubIr(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.HubIr))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.CameraHub(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.CameraHub))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.Camera(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.Camera))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.CameraPtz(device_info, handler):
                    name = name_or(device_info.nickname, str(DeviceType.CameraPtz))
                    ip, port = handler.address
                    print(f"Found '{name}' of model '{device_info.model}' at '{ip}:{port}'.")
                case DiscoveryResult.Other(device_info, (ip, port)):
                    # If you believe that this device is already supported through one of the existing handlers, or would like to explore adding support for a currently
                    # unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
                    # to start the discussion.
                    name = name_or(device_info.nickname or "", str(DeviceType.Other))
                    print(
                        f"Found device without a dedicated handler '{name}' of model '{device_info.model}' at '{ip}:{port}'."
                    )
        except Exception as e:
            print(f"Error discovering device: {e}")


def name_or(nickname: str, device_type: str) -> str:
    """Returns the device's name, or the device type when the name is empty. A
    device can have an empty name: the Tapo app doesn't show a power strip's
    own name, for example, but `set_nickname` can set it."""
    return nickname if nickname.strip() else device_type


if __name__ == "__main__":
    asyncio.run(main())
