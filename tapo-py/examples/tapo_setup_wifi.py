"""Wi-Fi setup of a new or factory-reset device Example

Join the device's setup network (e.g. `Tapo_Plug_999F`) first. The device
takes TAPO_USERNAME and TAPO_PASSWORD as its login from then on; they need
not be those of a TP-Link account.
"""

import asyncio
import os
from zoneinfo import ZoneInfo

from tapo import ApiClient

from common import require_env_vars


async def main():
    tapo_username, tapo_password, wifi_ssid, wifi_password, timezone = require_env_vars(
        "TAPO_USERNAME", "TAPO_PASSWORD", "WIFI_SSID", "WIFI_PASSWORD", "TIMEZONE"
    )
    ip_address = os.getenv("IP_ADDRESS", "192.168.0.1")

    client = ApiClient(tapo_username, tapo_password)
    setup = await client.setup_over_wifi(ip_address)

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
    print(f"Join result: {result}. Reconnect to {wifi_ssid} and discover the device there.")


if __name__ == "__main__":
    asyncio.run(main())
