"""P110, P110M and P115 Example"""

import asyncio
import os
from datetime import datetime, timedelta, timezone

from tapo import ApiClient
from tapo.requests import EnergyDataInterval, PowerDataInterval


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    device = await client.p110(ip_address)

    print("Turning device on...")
    await device.on()

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Turning device off...")
    await device.off()

    device_info = await device.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    current_power = await device.get_current_power()
    print(f"Current power: {current_power.to_dict()}")

    device_usage = await device.get_device_usage()
    print(f"Device usage: {device_usage.to_dict()}")

    energy_usage = await device.get_energy_usage()
    print(f"Energy usage: {energy_usage.to_dict()}")

    today = datetime.now(timezone.utc)

    # Energy data - Hourly interval
    # `start_date` and `end_date` are an inclusive interval that must not be greater than 8 days.
    energy_data_hourly = await device.get_energy_data(EnergyDataInterval.Hourly, today)
    print(
        "Energy data (hourly): "
        f"Start date time '{energy_data_hourly.start_date_time}', "
        f"Entries {len(energy_data_hourly.entries)}, "
        f"First entry: {energy_data_hourly.entries[0].to_dict() if energy_data_hourly.entries else None}"
    )

    # Energy data - Daily interval
    # `start_date` must be the first day of a quarter.
    energy_data_daily = await device.get_energy_data(
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
    energy_data_monthly = await device.get_energy_data(
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
    power_data_every_5_minutes = await device.get_power_data(
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
    power_data_hourly = await device.get_power_data(
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
