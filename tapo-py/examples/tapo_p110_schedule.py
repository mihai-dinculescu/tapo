"""Demo: schedule rules on a P110 (the "Schedule" feature in the Tapo app).

Adds a representative sample of rules — one-shot, weekly, sunset,
and sunrise — reads one of them back, then deletes them.  Existing
rules already on the device are left alone.

Build / run:
  cd tapo-py && maturin develop --release
  python examples/tapo_p110_schedule.py

Environment variables: TAPO_USERNAME, TAPO_PASSWORD, IP_ADDRESS.
"""

import asyncio
import logging
import os

from tapo import ApiClient
from tapo.requests import MON, WED, WEEKDAYS, EVERY_DAY, ScheduleRule


async def main():
    logging.basicConfig(level=logging.INFO, format="%(asctime)s %(message)s")
    log = logging.getLogger("schedule")

    tapo_username = os.getenv("TAPO_USERNAME")
    tapo_password = os.getenv("TAPO_PASSWORD")
    ip_address = os.getenv("IP_ADDRESS")

    client = ApiClient(tapo_username, tapo_password)
    device = await client.p110(ip_address)

    preexisting_ids = {r.id for r in await device.get_schedule_rules() if r.id}
    log.info("Pre-existing rules on the device: %d (left alone)", len(preexisting_ids))

    log.info("Adding four demo rules...")
    added = [
        # Turn on once, the next time the clock hits 06:30.
        await device.add_schedule_rule(ScheduleRule.clock_once(6, 30, True)),
        # Turn off weekly at 23:30 on Mondays and Wednesdays.
        await device.add_schedule_rule(ScheduleRule.clock_weekly(23, 30, MON | WED, False)),
        # Turn on every day, one hour after sunset.
        await device.add_schedule_rule(ScheduleRule.sunset_weekly(60, EVERY_DAY, True)),
        # Turn off on weekdays (Mon–Fri), 30 minutes before sunrise.
        await device.add_schedule_rule(ScheduleRule.sunrise_weekly(-30, WEEKDAYS, False)),
    ]
    added_ids = [r.id for r in added if r.id]
    log.info("  added ids: %s", added_ids)

    # Read the sunset rule (index 2) back and dump it.
    sunset_id = added[2].id
    assert sunset_id is not None
    rules_by_id = {r.id: r for r in await device.get_schedule_rules() if r.id}
    sunset = rules_by_id[sunset_id]
    log.info(
        "Read back sunset rule: id=%s time_kind=%s freq=%s "
        "offset_minutes=%d week_day=%s turn_on=%s",
        sunset.id,
        sunset.time_kind,
        sunset.frequency,
        sunset.offset_minutes,
        f"0b{sunset.week_day:07b}",
        sunset.turn_on,
    )

    log.info("Cleaning up: removing the four demo rules.")
    for rule_id in added_ids:
        await device.remove_schedule_rule(rule_id)

    remaining_ids = {r.id for r in await device.get_schedule_rules() if r.id}
    for rule_id in added_ids:
        assert (
            rule_id not in remaining_ids
        ), f"demo rule {rule_id} should be gone but is still on the device"
    log.info(
        "Cleanup OK — %d pre-existing rule(s) left intact: %s",
        len(remaining_ids),
        sorted(remaining_ids),
    )

    log.info("PASS")


if __name__ == "__main__":
    asyncio.run(main())
