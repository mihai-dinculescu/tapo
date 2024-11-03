"""KE100 TRV Example"""

import asyncio
import os

from tapo import ApiClient
from tapo.requests import TemperatureUnitKE100


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")
    # Name of the KE100 device.
    # Can be obtained from the Tapo App or by executing `get_child_device_component_list()` on the hub device.
    device_name = os.getenv("DEVICE_NAME")
    target_temperature = int(os.getenv("TARGET_TEMPERATURE"))

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
