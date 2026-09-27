"""H200 Example"""

import asyncio
from datetime import datetime, timedelta, timezone

from tapo import ApiClient
from tapo.responses import (
    KE100Result,
    OtherResult,
    S200Result,
    S210Result,
    T100Result,
    T110Result,
    T300Result,
    T31XResult,
)

from common import require_env_vars


async def main():
    tapo_username, tapo_password, ip_address = require_env_vars(
        "TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"
    )

    client = ApiClient(tapo_username, tapo_password)
    hub = await client.h200(ip_address)

    device_info = await hub.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    component_list = await hub.get_component_list()
    print(f"Component list: {[component.to_dict() for component in component_list]}")

    print("Getting child devices...")
    child_device_list = await hub.get_child_device_list()

    for child in child_device_list:
        if isinstance(child, OtherResult):
            print(
                "Found unsupported child device with nickname: {}, id: {}, model: {}.".format(
                    child.nickname, child.device_id, child.model
                )
            )
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
        elif isinstance(child, S200Result):
            s200 = await hub.s200(device_id=child.device_id)
            trigger_logs = await s200.get_trigger_logs(5, 0)

            print(
                "Found S200B/S200D child device with nickname: {}, id: {}, last 5 trigger logs: {}.".format(
                    child.nickname,
                    child.device_id,
                    [log.to_dict() for log in trigger_logs.logs],
                )
            )
        elif isinstance(child, S210Result):
            s210 = await hub.s210(device_id=child.device_id)
            device_usage = await s210.get_device_usage()

            print(
                "Found S210 child device with nickname: {}, id: {}, device_on: {}, device usage: {}.".format(
                    child.nickname,
                    child.device_id,
                    child.device_on,
                    device_usage.to_dict(),
                )
            )
        elif isinstance(child, T100Result):
            t100 = await hub.t100(device_id=child.device_id)
            trigger_logs = await t100.get_trigger_logs(5, 0)

            print(
                "Found T100 child device with nickname: {}, id: {}, detected: {}, last 5 trigger logs: {}.".format(
                    child.nickname,
                    child.device_id,
                    child.detected,
                    [log.to_dict() for log in trigger_logs.logs],
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
            t300 = await hub.t300(device_id=child.device_id)
            trigger_logs = await t300.get_trigger_logs(5, 0)

            print(
                "Found T300 child device with nickname: {}, id: {}, in_alarm: {}, water_leak_status: {}, last 5 trigger logs: {}.".format(
                    child.nickname,
                    child.device_id,
                    child.in_alarm,
                    child.water_leak_status,
                    [log.to_dict() for log in trigger_logs.logs],
                )
            )
        elif isinstance(child, T31XResult):
            t31x = await hub.t31x(device_id=child.device_id)
            temperature_humidity_records = await t31x.get_temperature_humidity_records()

            print(
                "Found T310/T315 child device with nickname: {}, id: {}, temperature: {:.2f} {}, humidity: {}%, earliest temperature and humidity record available: {}.".format(
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

    general_device_list = await hub.get_general_device_list()

    # The first recording found for each camera, to download at the end.
    recordings_to_download = []

    for general_device in general_device_list:
        print(
            "Found general device with alias: {}, id: {}, model: {}, hub storage enabled: {}.".format(
                general_device.alias,
                general_device.device_id,
                general_device.device_model,
                general_device.hub_storage_enabled,
            )
        )

        if not general_device.hub_storage_enabled:
            continue

        end_time = datetime.now(timezone.utc)
        start_time = end_time - timedelta(days=7)

        recording_dates = await hub.get_recording_dates(
            general_device.device_id, start_time, end_time
        )
        print(
            "{} has recordings stored on the hub on the following dates: {}.".format(
                general_device.alias,
                [recording_date.to_dict() for recording_date in recording_dates],
            )
        )

        if recording_dates:
            recording_date = recording_dates[-1]
            recordings = await hub.get_recordings(
                general_device.device_id,
                recording_date.start_time,
                recording_date.end_time,
            )
            print(
                "{} has {} recordings on {}. First: {}.".format(
                    general_device.alias,
                    len(recordings),
                    recording_date.date,
                    recordings[0].to_dict() if recordings else None,
                )
            )

            if recordings:
                recordings_to_download.append((general_device.device_id, recordings[0]))

    if not recordings_to_download:
        print("No recording found to download.")

    for device_id, recording in recordings_to_download:
        path = f"recording_{device_id}_{int(recording.start_time.timestamp())}.ts"
        print(
            "Downloading the recording from {} to {} of {} to {}...".format(
                recording.start_time, recording.end_time, device_id, path
            )
        )

        try:
            result = await hub.download_recording(
                device_id, recording.start_time, recording.end_time, path
            )
        except Exception as err:
            print(f"Failed to download the recording of {device_id}: {err}")
            continue

        duration = (
            f"{result.duration_s:.3f} s" if result.duration_s is not None else "an unknown length"
        )
        print(f"Wrote {result.byte_count} bytes to {path}: {duration} of video.")


if __name__ == "__main__":
    asyncio.run(main())
