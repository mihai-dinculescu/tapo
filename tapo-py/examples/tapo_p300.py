"""P300 and P304 Example"""

import asyncio
import os

from tapo import ApiClient


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    power_strip = await client.p300(ip_address)

    device_info = await power_strip.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    child_device_list = await power_strip.get_child_device_list()

    for child in child_device_list:
        print(
            "Found plug with nickname: {}, id: {}, state: {}.".format(
                child.nickname,
                child.device_id,
                child.device_on,
            )
        )

        plug = await power_strip.plug(device_id=child.device_id)

        print("Turning device on...")
        await plug.on()

        print("Waiting 2 seconds...")
        await asyncio.sleep(2)

        print("Turning device off...")
        await plug.off()

        print("Waiting 2 seconds...")
        await asyncio.sleep(2)


if __name__ == "__main__":
    asyncio.run(main())
