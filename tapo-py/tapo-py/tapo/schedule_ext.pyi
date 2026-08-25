from typing import List, Protocol

from tapo.requests import ScheduleRule
from tapo.responses import ScheduleRuleResult

class ScheduleExt(Protocol):
    """Extension class for the plug's schedule rules (the "Schedule" feature
    in the Tapo app). Schedule rules live on the device, so they keep firing
    even if the phone / Wi-Fi router / Tapo cloud is offline."""

    async def add_schedule_rule(self, rule: ScheduleRule) -> ScheduleRuleResult:
        """Adds a new schedule rule. Returns it as a ``ScheduleRuleResult``
        carrying the device-assigned ``id``.

        Args:
            rule: the rule to add; build one with the ``ScheduleRule``
                factories. Any ``id`` it carries is ignored, because the
                device assigns one.

        The device has a fixed capacity — a P110 stores 32 rules — and raises
        once it is full.
        """

    async def edit_schedule_rule(self, rule: ScheduleRule) -> None:
        """Edits an existing rule, replacing it with the given one.

        Args:
            rule: the replacement rule. Its ``id`` must be set to the id of
                the rule to update, either from ``add_schedule_rule`` /
                ``get_schedule_rules`` or via ``ScheduleRule.with_id``. An id
                the device does not know raises.
        """

    async def get_schedule_rules(self) -> List[ScheduleRuleResult]:
        """Returns every schedule rule currently stored on the device. A rule
        the library cannot parse is skipped rather than failing the whole
        listing."""

    async def get_max_schedule_rules(self) -> int:
        """Returns how many schedule rules the device can store in total, as
        reported by the device itself. A P110 stores 32. Compare against the
        length of ``get_schedule_rules`` to tell whether there is room for
        another rule, because ``add_schedule_rule`` raises once the device is
        full."""

    async def remove_schedule_rule(self, id: str) -> None:
        """Removes a single schedule rule, leaving every other rule in place.

        Args:
            id: the device-assigned id of the rule to remove.
        """

    async def remove_all_schedule_rules(self) -> None:
        """Removes every schedule rule from the device."""
