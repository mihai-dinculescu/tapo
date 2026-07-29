"""P110, P110M and P115 Timer Example"""

import asyncio

from tapo import ApiClient
from tapo.responses import PowerState

from common import require_env_vars


async def main():
    tapo_username, tapo_password, ip_address = require_env_vars(
        "TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"
    )

    client = ApiClient(tapo_username, tapo_password)
    device = await client.p110(ip_address)

    print("Turning device off and clearing any armed timer...")
    await device.off()
    await device.clear_timer()

    # The delay must be between 1 second and 24 hours.
    print("Arming a 5 second timer that turns the device on...")
    timer = await device.set_timer(5, PowerState.On)
    print(f"Armed timer: {timer.to_dict()}")

    timer = await device.get_timer()
    print(f"Timer: {timer.to_dict() if timer else None}")

    print("Waiting 10 seconds for the timer to fire...")
    await asyncio.sleep(10)

    device_info = await device.get_device_info()
    print(f"Device on: {device_info.device_on}")

    timer = await device.get_timer()
    print(f"Timer once fired: {timer.to_dict() if timer else None}")

    print("Arming a 5 second timer that turns the device off...")
    timer = await device.set_timer(5, PowerState.Off)
    print(f"Armed timer: {timer.to_dict()}")

    print("Clearing the timer before it fires...")
    await device.clear_timer()

    timer = await device.get_timer()
    print(f"Timer once cleared: {timer.to_dict() if timer else None}")

    print("Waiting 10 seconds to show that the cleared timer does not fire...")
    await asyncio.sleep(10)

    device_info = await device.get_device_info()
    print(f"Device on: {device_info.device_on}")

    print("Turning device off...")
    await device.off()


if __name__ == "__main__":
    asyncio.run(main())
