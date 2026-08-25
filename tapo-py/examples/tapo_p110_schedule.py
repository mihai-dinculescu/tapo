"""P110, P110M and P115 Schedule Example"""

import asyncio

from tapo import ApiClient
from tapo.requests import ScheduleRule, DaysOfWeek
from tapo.responses import PowerState

from common import require_env_vars


async def main():
    tapo_username, tapo_password, ip_address = require_env_vars(
        "TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"
    )

    client = ApiClient(tapo_username, tapo_password)
    device = await client.p110(ip_address)

    # Schedule rules fire on the device's own clock, so this example adds,
    # edits and removes rules instead of waiting for one of them to fire.
    # Rules that are already on the device are left alone; only the four
    # rules added below are removed again at the end.
    rules = await device.get_schedule_rules()
    max_rules = await device.get_max_schedule_rules()
    print(f"The device starts with {len(rules)} of a maximum {max_rules} schedule rules.")
    print(f"There is room for {max(0, max_rules - len(rules))} more.")

    print("Adding a rule that turns the device on once, at the next 06:30...")
    rule = ScheduleRule.clock_once(6, 30, PowerState.On)
    morning = await device.add_schedule_rule(rule)
    print(f"Added rule: {morning.to_dict()}")

    print("Adding a rule that turns the device off at 23:30 on Mondays and Wednesdays...")
    rule = ScheduleRule.clock_weekly(23, 30, DaysOfWeek.MON | DaysOfWeek.WED, PowerState.Off)
    late_night = await device.add_schedule_rule(rule)
    print(f"Added rule: {late_night.to_dict()}")

    print("Adding a rule that turns the device on every day, an hour after sunset...")
    rule = ScheduleRule.sunset_weekly(60, DaysOfWeek.EVERY_DAY, PowerState.On)
    after_sunset = await device.add_schedule_rule(rule)
    print(f"Added rule: {after_sunset.to_dict()}")

    # A negative offset fires before the astronomical event instead of after it.
    print("Adding a rule that turns the device off on weekdays, 30 minutes before sunrise...")
    rule = ScheduleRule.sunrise_weekly(-30, DaysOfWeek.WEEKDAYS, PowerState.Off)
    before_sunrise = await device.add_schedule_rule(rule)
    print(f"Added rule: {before_sunrise.to_dict()}")

    rules = await device.get_schedule_rules()
    print(f"The four rules should have been added: the device now holds {len(rules)} rules.")
    for rule in rules:
        print(f"Rule: {rule.to_dict()}")

    # Rules come back as results, which are read-only; `to_editable` turns one
    # back into a rule that can be changed and sent. A disabled rule stays on
    # the device, but does not fire.
    print("Disabling the 23:30 rule...")
    await device.edit_schedule_rule(late_night.to_editable().with_enabled(False))
    rules = await device.get_schedule_rules()
    late_night_after_edit = next((rule for rule in rules if rule.id == late_night.id), None)
    print(
        "The 23:30 rule should now be disabled: "
        f"{late_night_after_edit.to_dict() if late_night_after_edit else None}"
    )

    print("Removing the four rules that were added...")
    for rule in [morning, late_night, after_sunset, before_sunrise]:
        await device.remove_schedule_rule(rule.id)

    rules = await device.get_schedule_rules()
    print(f"The added rules should be gone: the device holds {len(rules)} rules again.")


if __name__ == "__main__":
    asyncio.run(main())
