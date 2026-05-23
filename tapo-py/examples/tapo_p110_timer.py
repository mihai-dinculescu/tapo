"""Demo: arming the plug's countdown ("Timer" in the Tapo app).

Build / run:
  cd tapo-py && maturin develop --release
  python examples/tapo_p110_timer.py

Environment variables: TAPO_USERNAME, TAPO_PASSWORD, IP_ADDRESS.
"""

import asyncio
import logging
import os

from tapo import ApiClient


async def main():
    logging.basicConfig(level=logging.INFO, format="%(asctime)s %(message)s")
    log = logging.getLogger("timer")

    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    device = await client.p110(ip_address)

    log.info("Baseline: plug off, no armed timer.")
    await device.off()
    await device.clear_timer()
    assert await device.get_timer() is None

    log.info("Arming a 10-second 'turn ON' timer...")
    armed = await device.set_timer(10, True)
    log.info("Armed: id=%s delay=%ds", armed.id, armed.delay_seconds)

    read_back = await device.get_timer()
    assert read_back is not None
    log.info(
        "Read back: id=%s remain=%ds turn_on=%s",
        read_back.id,
        read_back.remaining_seconds,
        read_back.turn_on,
    )
    assert read_back.id == armed.id
    assert read_back.turn_on is True

    log.info("Waiting 15 seconds for the timer to fire (10s delay + slack)...")
    await asyncio.sleep(15)
    assert (await device.get_device_info()).device_on, "plug should be ON"
    log.info("Timer fired — plug is ON.")

    log.info("Arming a 5-second 'turn OFF' timer and clearing it before it fires...")
    await device.set_timer(5, False)
    await device.clear_timer()
    assert await device.get_timer() is None

    log.info("Waiting 10 seconds to confirm the cleared timer did not fire...")
    await asyncio.sleep(10)
    assert (
        await device.get_device_info()
    ).device_on, "plug should still be on after the cleared timer's original deadline"

    await device.off()
    log.info("PASS")


if __name__ == "__main__":
    asyncio.run(main())
