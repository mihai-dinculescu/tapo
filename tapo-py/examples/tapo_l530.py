"""L530, L630 & L900 Example"""

import asyncio
import os

from tapo import ApiClient, Color


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    device = await client.l530(ip_address)

    print("Turning device on...")
    await device.on()

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Setting the brightness to 30%...")
    await device.set_brightness(30)

    print("Setting the color to `Chocolate`...")
    await device.set_color(Color.Chocolate)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Setting the color to `Deep Sky Blue` using the `hue` and `saturation`...")
    await device.set_hue_saturation(195, 100)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Setting the color to `Incandescent` using the `color temperature`...")
    await device.set_color_temperature(2700)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Using the `set` API to set multiple properties in a single request...")
    await device.set().brightness(50).color(Color.HotPink).send(device)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Turning device off...")
    await device.off()

    device_info = await device.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    device_usage = await device.get_device_usage()
    print(f"Device usage: {device_usage.to_dict()}")


if __name__ == "__main__":
    asyncio.run(main())
