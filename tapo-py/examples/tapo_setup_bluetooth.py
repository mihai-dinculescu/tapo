"""Bluetooth setup of a new or factory-reset device Example

The device takes TAPO_USERNAME and TAPO_PASSWORD as its login from then on;
they need not be those of a TP-Link account.
"""

import asyncio
from zoneinfo import ZoneInfo

from tapo import ApiClient

from common import require_env_vars


async def main():
    tapo_username, tapo_password, wifi_ssid, wifi_password, timezone = require_env_vars(
        "TAPO_USERNAME", "TAPO_PASSWORD", "WIFI_SSID", "WIFI_PASSWORD", "TIMEZONE"
    )

    print("Scanning for devices to set up over Bluetooth...")
    devices = await ApiClient.discover_bluetooth_setup_devices(10)
    for device in devices:
        print(f"Device: {device.to_dict()}")
    if not devices:
        raise RuntimeError("No device to set up over Bluetooth was found")

    client = ApiClient(tapo_username, tapo_password)
    setup = await client.setup_over_bluetooth(devices[0])

    device_info = await setup.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    print("Scanning for Wi-Fi networks...")
    networks = await setup.scan_wifi()
    for network in networks:
        print(f"Network: {network.to_dict()}")

    network = next((network for network in networks if network.ssid == wifi_ssid), None)
    if network is None:
        raise RuntimeError(f"The device does not see the Wi-Fi network {wifi_ssid}")

    print(f"Joining {wifi_ssid}...")
    result = await setup.join_wifi(wifi_ssid, wifi_password, network.security, ZoneInfo(timezone))
    print(f"Join result: {result}. Discover the device on {wifi_ssid}.")


if __name__ == "__main__":
    asyncio.run(main())
