"""H100 Example"""

import asyncio
import os

from tapo import ApiClient
from tapo.responses import KE100Result, S200BResult, T100Result, T110Result, T300Result, T31XResult


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    hub = await client.h100(ip_address)

    device_info = await hub.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    child_device_list = await hub.get_child_device_list()

    for child in child_device_list:
        if child is None:
            print("Found unsupported device.")
        elif isinstance(child, KE100Result):
            print(
                "Found KE100 child device with nickname: {}, id: {}, current temperature: {:.2f} {} and target temperature: {:.2f} {}.".format(
                    child.nickname,
                    child.device_id,
                    child.current_temperature,
                    child.temperature_unit,
                    child.target_temperature,
                    child.temperature_unit,
                )
            )
        elif isinstance(child, S200BResult):
            print(
                "Found S200B child device with nickname: {}, id: {}.".format(
                    child.nickname,
                    child.device_id,
                )
            )
        elif isinstance(child, T100Result):
            print(
                "Found T100 child device with nickname: {}, id: {}, detected: {}.".format(
                    child.nickname,
                    child.device_id,
                    child.detected,
                )
            )
        elif isinstance(child, T110Result):
            t110 = await hub.t110(device_id=child.device_id)
            trigger_logs = await t110.get_trigger_logs(5, 0)

            print(
                "Found T110 child device with nickname: {}, id: {}, open: {}, last 5 trigger logs: {}.".format(
                    child.nickname,
                    child.device_id,
                    child.open,
                    [log.to_dict() for log in trigger_logs.logs],
                )
            )
        elif isinstance(child, T300Result):
            print(
                "Found T300 child device with nickname: {}, id: {}, in_alarm: {}, water_leak_status: {}.".format(
                    child.nickname, child.device_id, child.in_alarm, child.water_leak_status
                )
            )
        elif isinstance(child, T31XResult):
            t31x = await hub.t315(device_id=child.device_id)
            temperature_humidity_records = await t31x.get_temperature_humidity_records()

            print(
                "Found T31X child device with nickname: {}, id: {}, temperature: {:.2f} {}, humidity: {}%, earliest temperature and humidity record available: {}.".format(
                    child.nickname,
                    child.device_id,
                    child.current_temperature,
                    child.temperature_unit,
                    child.current_humidity,
                    (
                        temperature_humidity_records.records[0].to_dict()
                        if temperature_humidity_records.records
                        else None
                    ),
                )
            )


if __name__ == "__main__":
    asyncio.run(main())
