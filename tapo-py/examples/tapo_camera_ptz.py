"""Tapo cameras with PTZ (C210, C220, C225, C325WB, C520WS, TC40, TC70) Example"""

import asyncio
import os

from tapo import ApiClient


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")
    camera_username = os.getenv("TAPO_CAMERA_USERNAME")
    camera_password = os.getenv("TAPO_CAMERA_PASSWORD")

    client = ApiClient(tapo_username, tapo_password)
    device = await client.c220(ip_address)

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

    preset_name = f"example_{os.getpid()}"

    print(f"Saving current position as preset '{preset_name}'...")
    await device.save_preset(preset_name)

    presets = await device.get_presets()
    print(f"Presets: {[p.to_dict() for p in presets]}")

    preset_id = next(p.id for p in presets if p.name == preset_name)

    print("Panning and tilting by 10, 10...")
    await device.pan_tilt(10, 10)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print(f"Going back to saved preset (id '{preset_id}')...")
    await device.goto_preset(preset_id)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print(f"Deleting preset (id '{preset_id}')...")
    await device.delete_preset(preset_id)


if __name__ == "__main__":
    asyncio.run(main())
