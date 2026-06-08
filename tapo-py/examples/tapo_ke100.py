"""KE100 TRV Example"""

import asyncio

from tapo import ApiClient
from tapo.requests import TemperatureUnitKE100

from common import require_env_vars


async def main():
    # `DEVICE_NAME` is the name of the KE100 device.
    # It can be obtained from the Tapo App or by executing `get_child_device_component_list()` on the hub device.
    tapo_username, tapo_password, ip_address, device_name, target_temperature_str = (
        require_env_vars(
            "TAPO_USERNAME",
            "TAPO_PASSWORD",
            "IP_ADDRESS",
            "DEVICE_NAME",
            "TARGET_TEMPERATURE",
        )
    )
    target_temperature = int(target_temperature_str)

    client = ApiClient(tapo_username, tapo_password)
    hub = await client.h100(ip_address)

    # Get a handler for the child device
    device = await hub.ke100(nickname=device_name)

    # Get the device info of the child device
    device_info = await device.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    # Set target temperature.
    # KE100 currently only supports Celsius as temperature unit.
    print(f"Setting target temperature to {target_temperature} degrees Celsius...")
    await device.set_target_temperature(target_temperature, TemperatureUnitKE100.Celsius)

    # Get the device info of the child device
    device_info = await device.get_device_info()
    print(f"Device info: {device_info.to_dict()}")


if __name__ == "__main__":
    asyncio.run(main())
