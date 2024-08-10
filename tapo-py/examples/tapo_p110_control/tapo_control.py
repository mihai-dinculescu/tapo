"""Tapo P110 & P115 Control"""

import argparse
import asyncio
import os
from datetime import datetime
from dotenv import load_dotenv

from tapo import ApiClient
from tapo.requests import EnergyDataInterval

# Load credentials from .env
load_dotenv()

def get_quarter_start_month(today: datetime) -> int:
    return 3 * ((today.month - 1) // 3) + 1

async def main(ip_address, action):
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")

    if not tapo_username or not tapo_password:
        print("Error: Please set TAPO_USERNAME and TAPO_PASSWORD in the .env file.")
        return

    try:
        client = ApiClient(tapo_username, tapo_password)
        device = await client.p110(ip_address)
    except Exception as e:
        print(f"Error initializing device at {ip_address}: {e}")
        return

    try:
        if action == "on":
            print(f"Turning device at {ip_address} on...")
            await device.on()

        elif action == "off":
            print(f"Turning device at {ip_address} off...")
            await device.off()

        elif action == "device_reset":
            print(f"Resetting device at {ip_address}...")
            await device.reset()

        elif action == "get_device_info":
            device_info = await device.get_device_info()
            print(f"Device info: {device_info.to_dict()}")

        elif action == "get_device_info_json":
            device_info = await device.get_device_info()
            print(device_info.to_json())

        elif action == "get_device_usage":
            device_usage = await device.get_device_usage()
            print(f"Device usage: {device_usage.to_dict()}")

        elif action == "get_current_power":
            current_power = await device.get_current_power()
            print(f"Current power: {current_power.to_dict()}")

        elif action == "get_energy_usage":
            energy_usage = await device.get_energy_usage()
            print(f"Energy usage: {energy_usage.to_dict()}")

        elif action == "get_energy_data":
            today = datetime.today()
            energy_data_hourly = await device.get_energy_data(EnergyDataInterval.Hourly, today)
            print(f"Energy data (hourly): {energy_data_hourly.to_dict()}")

            energy_data_daily = await device.get_energy_data(
                EnergyDataInterval.Daily,
                datetime(today.year, get_quarter_start_month(today), 1),
            )
            print(f"Energy data (daily): {energy_data_daily.to_dict()}")

            energy_data_monthly = await device.get_energy_data(
                EnergyDataInterval.Monthly,
                datetime(today.year, 1, 1),
            )
            print(f"Energy data (monthly): {energy_data_monthly.to_dict()}")

        elif action == "refresh_session":
            print(f"Refreshing session for device at {ip_address}...")
            await device.refresh_session()

        else:
            print(f"Unknown action: {action}")

    except Exception as e:
        print(f"Error performing action {action} on device at {ip_address}: {e}")

class CustomHelpFormatter(argparse.RawTextHelpFormatter):
    def _format_action_invocation(self, action):
        if not action.option_strings:
            return f'  {action.dest}'
        return super()._format_action_invocation(action)

    def _format_args(self, action, default_metavar):
        if action.choices:
            choices_str = ', '.join(action.choices)
            return f'{{{choices_str}}}'
        return default_metavar

    def add_arguments(self, actions):
        actions = sorted(actions, key=lambda x: x.option_strings[0] if x.option_strings else x.dest)
        super().add_arguments(actions)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Control Tapo devices.",
        formatter_class=CustomHelpFormatter
    )
    parser.add_argument(
        "ip_address",
        type=str,
        help="IP address of the Tapo device"
    )
    parser.add_argument(
        "action",
        type=str,
        help="Command to perform on the device. Available commands are:\n"
             "  device_reset        - Reset the device\n"
             "  get_current_power   - Get the current power consumption\n"
             "  get_device_info     - Get detailed device information\n"
             "  get_device_info_json- Get detailed device information in JSON format\n"
             "  get_device_usage    - Get the device usage statistics\n"
             "  get_energy_data     - Get energy data at different intervals\n"
             "  get_energy_usage    - Get the energy usage statistics\n"
             "  off                 - Turn the device off\n"
             "  on                  - Turn the device on\n"
             "  refresh_session     - Refresh the session with the device\n",
        choices=[
            "device_reset",
            "get_current_power",
            "get_device_info",
            "get_device_info_json",
            "get_device_usage",
            "get_energy_data",
            "get_energy_usage",
            "off",
            "on",
            "refresh_session"
        ]
    )

    args = parser.parse_args()

    asyncio.run(main(args.ip_address, args.action))
