"""Tapo cameras without PTZ (C120) Example"""

import asyncio
import os

from tapo import ApiClient

from common import require_env_vars


async def main():
    tapo_username, tapo_password, ip_address, camera_username, camera_password = require_env_vars(
        "TAPO_USERNAME",
        "TAPO_PASSWORD",
        "IP_ADDRESS",
        "TAPO_CAMERA_USERNAME",
        "TAPO_CAMERA_PASSWORD",
    )

    client = ApiClient(tapo_username, tapo_password)
    device = await client.c120(ip_address)

    device_info = await device.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    rtsp_url = await device.get_rtsp_stream_url(camera_username, camera_password)
    print(f"RTSP HD: {rtsp_url.hd}")
    print(f"RTSP SD: {rtsp_url.sd}")
    print(f"RTSP MJPEG: {rtsp_url.mjpeg}")

    print("Capturing snapshot...")
    snapshot = await device.get_snapshot(camera_username, camera_password)
    snapshot_path = f"snapshot_{os.getpid()}.jpg"
    with open(snapshot_path, "wb") as f:
        f.write(snapshot.data)
    print(
        f"Saved snapshot ({len(snapshot.data)} bytes, {snapshot.content_type}) "
        f"to {snapshot_path}"
    )


if __name__ == "__main__":
    asyncio.run(main())
