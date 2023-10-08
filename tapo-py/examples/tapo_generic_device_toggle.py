"""Toggle Generic Device Example"""

import asyncio
import os

from tapo import ApiClient


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    device = await client.generic_device(ip_address)

    device_info = await device.get_device_info()

    if device_info.device_on == True:
        print("Device is on. Turning it off...")
        await device.off()
    elif device_info.device_on == False:
        print("Device is off. Turning it on...")
        await device.on()
    else:
        print("This device does not support on/off functionality.")


if __name__ == "__main__":
    asyncio.run(main())
