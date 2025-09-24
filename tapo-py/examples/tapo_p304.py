"""P304M and P316M Example"""

import asyncio
from datetime import datetime, timedelta, timezone
import os

from tapo import ApiClient
from tapo.requests import EnergyDataInterval, PowerDataInterval


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    power_strip = await client.p304(ip_address)

    device_info = await power_strip.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    print("Getting child devices...")
    child_device_list = await power_strip.get_child_device_list()
    print(f"Found {len(child_device_list)} plugs")

    for index, child in enumerate(child_device_list):
        print(f"=== ({index + 1}) {child.nickname} ===")
        print(f"Device ID: {child.device_id}")
        print(f"State: {child.device_on}")

        plug = await power_strip.plug(device_id=child.device_id)

        print("Turning device on...")
        await plug.on()

        print("Waiting 2 seconds...")
        await asyncio.sleep(2)

        print("Turning device off...")
        await plug.off()

        print("Waiting 2 seconds...")
        await asyncio.sleep(2)

        current_power = await plug.get_current_power()
        print(f"Current power: {current_power.to_dict()}")

        device_usage = await plug.get_device_usage()
        print(f"Device usage: {device_usage.to_dict()}")

        energy_usage = await plug.get_energy_usage()
        print(f"Energy usage: {energy_usage.to_dict()}")

        today = datetime.now(timezone.utc)

        # Energy data - Hourly interval
        # `start_date` and `end_date` are an inclusive interval that must not be greater than 8 days.
        energy_data_hourly = await plug.get_energy_data(EnergyDataInterval.Hourly, today)
        print(
            "Energy data (hourly): "
            f"Start date time '{energy_data_hourly.start_date_time}', "
            f"Entries {len(energy_data_hourly.entries)}, "
            f"First entry: {energy_data_hourly.entries[0].to_dict() if energy_data_hourly.entries else None}"
        )

        # Energy data - Daily interval
        # `start_date` must be the first day of a quarter.
        energy_data_daily = await plug.get_energy_data(
            EnergyDataInterval.Daily,
            datetime(today.year, get_quarter_start_month(today), 1),
        )
        print(
            "Energy data (daily): "
            f"Start date time '{energy_data_daily.start_date_time}', "
            f"Entries {len(energy_data_daily.entries)}, "
            f"First entry: {energy_data_daily.entries[0].to_dict() if energy_data_daily.entries else None}"
        )

        # Energy data - Monthly interval
        # `start_date` must be the first day of a year.
        energy_data_monthly = await plug.get_energy_data(
            EnergyDataInterval.Monthly,
            datetime(today.year, 1, 1),
        )
        print(
            "Energy data (monthly): "
            f"Start date time '{energy_data_monthly.start_date_time}', "
            f"Entries {len(energy_data_monthly.entries)}, "
            f"First entry: {energy_data_monthly.entries[0].to_dict() if energy_data_monthly.entries else None}"
        )

        # Power data - Every 5 minutes interval
        # `start_date_time` and `end_date_time` describe an exclusive interval.
        # If the result would yield more than 144 entries (i.e. 12 hours),
        # the `end_date_time` will be adjusted to an earlier date and time.
        power_data_every_5_minutes = await plug.get_power_data(
            PowerDataInterval.Every5Minutes,
            today - timedelta(hours=12),
            today,
        )
        print(
            "Power data (every 5 minutes): "
            f"Start date time '{power_data_every_5_minutes.start_date_time}', "
            f"End date time '{power_data_every_5_minutes.end_date_time}', "
            f"Entries {len(power_data_every_5_minutes.entries)}, "
            f"First entry: {power_data_every_5_minutes.entries[0].to_dict() if power_data_every_5_minutes.entries else None}"
        )

        # Power data - Hourly interval
        # `start_date_time` and `end_date_time` describe an exclusive interval.
        # If the result would yield more than 144 entries (i.e. 6 days),
        # the `end_date_time` will be adjusted to an earlier date and time.
        power_data_hourly = await plug.get_power_data(
            PowerDataInterval.Hourly,
            today - timedelta(days=3),
            today,
        )
        print(
            "Power data (hourly): "
            f"Start date time '{power_data_hourly.start_date_time}', "
            f"End date time '{power_data_hourly.end_date_time}', "
            f"Entries {len(power_data_hourly.entries)}, "
            f"First entry: {power_data_hourly.entries[0].to_dict() if power_data_hourly.entries else None}"
        )


def get_quarter_start_month(today: datetime) -> int:
    return ((today.month - 1) // 3) * 3 + 1


if __name__ == "__main__":
    asyncio.run(main())
