"""L920 and L930 Example"""

import asyncio
import os

from tapo import ApiClient
from tapo.requests import Color, LightingEffect, LightingEffectPreset, LightingEffectType


async def main():
    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    device = await client.l930(ip_address)

    print("Turning device on...")
    await device.on()

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Setting the brightness to 30%...")
    await device.set_brightness(30)

    print("Setting the color to `Chocolate`...")
    await device.set_color(Color.Chocolate)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Setting the color to `Deep Sky Blue` using the `hue` and `saturation`...")
    await device.set_hue_saturation(195, 100)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Setting the color to `Incandescent` using the `color temperature`...")
    await device.set_color_temperature(2700)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Using the `set` API to set multiple properties in a single request...")
    await device.set().brightness(50).color(Color.HotPink).send(device)

    print("Waiting 2 seconds...")
    await asyncio.sleep(2)

    print("Setting a preset Lighting effect...")
    await device.set_lighting_effect(LightingEffectPreset.BubblingCauldron)

    print("Waiting 10 seconds...")
    await asyncio.sleep(10)

    print("Setting a custom static Lighting effect...")
    custom_effect = (
        LightingEffect(
            "My Custom Static Effect", LightingEffectType.Static, True, True, 100, [(359, 85, 100)]
        )
        .with_expansion_strategy(1)
        .with_segments([0, 1, 2])
        .with_sequence([(359, 85, 100), (0, 0, 100), (236, 72, 100)])
    )
    await device.set_lighting_effect(custom_effect)

    print("Waiting 10 seconds...")
    await asyncio.sleep(10)

    print("Setting a custom sequence Lighting effect...")
    custom_effect = (
        LightingEffect(
            "My Custom Sequence Effect",
            LightingEffectType.Sequence,
            True,
            True,
            100,
            [(359, 85, 100)],
        )
        .with_expansion_strategy(1)
        .with_segments([0, 1, 2])
        .with_sequence([(359, 85, 100), (0, 0, 100), (236, 72, 100)])
        .with_direction(1)
        .with_duration(50)
    )
    await device.set_lighting_effect(custom_effect)

    print("Waiting 10 seconds...")
    await asyncio.sleep(10)

    print("Turning device off...")
    await device.off()

    device_info = await device.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    device_usage = await device.get_device_usage()
    print(f"Device usage: {device_usage.to_dict()}")


if __name__ == "__main__":
    asyncio.run(main())
