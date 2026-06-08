"""P100 and P105 Example"""

import asyncio

from tapo import ApiClient

from common import require_env_vars


async def main():
    tapo_username, tapo_password, ip_address = require_env_vars(
        "TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"
    )

    client = ApiClient(tapo_username, tapo_password)
    device = await client.p100(ip_address)

    print("Turning device on...")
    await device.on()

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
