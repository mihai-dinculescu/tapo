from typing import Optional

from tapo.requests import DaysOfWeek, ScheduleRule, ScheduleTime
from tapo.responses import PowerState
from tapo.to_dict_ext import ToDictExt

class ScheduleRuleResult(ToDictExt):
    """A plug schedule rule read back from the device (the "Schedule" feature
    in the Tapo app).

    This is the lenient counterpart of ``ScheduleRule``: it reports whatever
    the device holds, so a rule written by a newer app or firmware does not
    stop the rest of the listing from being read. Convert one into a validated
    ``ScheduleRule`` with ``to_editable`` to edit it."""

    id: str
    """Device-assigned id."""
    enabled: bool
    """Whether the rule is currently active. Disabled rules are kept on the
    device but do not fire."""
    time: ScheduleTime
    """When the rule fires within a day."""
    days: Optional[DaysOfWeek]
    """The days a repeating rule fires on, or ``None`` when it fires once."""
    desired_state: PowerState
    """The state the plug transitions to when the rule fires."""

    def to_editable(self) -> ScheduleRule:
        """Returns this rule as a validated ``ScheduleRule``, carrying its id
        across, ready to be changed with ``with_*`` and passed to
        ``edit_schedule_rule``.

        Raises because this type is parsed leniently and may hold values the
        write type refuses, such as a repeating rule the device stored with no
        days set."""
