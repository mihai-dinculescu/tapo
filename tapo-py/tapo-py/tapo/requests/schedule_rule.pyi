from enum import Enum
from typing import Optional

from tapo.to_dict_ext import ToDictExt

# Day-of-week bitmask constants (bit 0 = Sun, bit 6 = Sat). Combine
# with ``|`` and pass as the ``week_day`` argument of the ``*_weekly``
# factory methods.
SUN: int
"""Sunday."""
MON: int
"""Monday."""
TUE: int
"""Tuesday."""
WED: int
"""Wednesday."""
THU: int
"""Thursday."""
FRI: int
"""Friday."""
SAT: int
"""Saturday."""
WEEKDAYS: int
"""Monday through Friday."""
WEEKEND: int
"""Saturday and Sunday."""
EVERY_DAY: int
"""Every day of the week."""

class ScheduleTimeKind(str, Enum):
    """When a ``ScheduleRule`` fires within a day."""

    Clock = "Clock"
    """At a wall-clock time (minutes after midnight, device local time)."""
    Sunrise = "Sunrise"
    """At a fixed offset from civil sunrise."""
    Sunset = "Sunset"
    """At a fixed offset from civil sunset."""

class ScheduleFrequency(str, Enum):
    """Whether a ``ScheduleRule`` fires once or repeats weekly."""

    Once = "Once"
    """Fires once, at the next matching time."""
    Weekly = "Weekly"
    """Fires every day matched by the rule's ``week_day`` bitmask."""

class ScheduleRule(ToDictExt):
    """A plug schedule rule (the "Schedule" feature in the Tapo app).

    Construct one with the factory classmethods below.  Day-of-week
    bitmasks: combine the ``SUN`` / ``MON`` / ... / ``SAT`` constants
    in this module (or use ``WEEKDAYS`` / ``WEEKEND`` / ``EVERY_DAY``)
    as the ``week_day`` argument of the ``*_weekly`` factories."""

    id: Optional[str]
    """Device-assigned id.  ``None`` when constructed locally."""
    enable: bool
    """Whether the rule is currently active."""
    time_kind: ScheduleTimeKind
    minute_of_day: int
    """For Clock rules: minutes after midnight (0..1440)."""
    offset_minutes: int
    """For sunrise / sunset rules: signed minutes from the event."""
    frequency: ScheduleFrequency
    week_day: int
    """Bitmask of days the rule fires on, when ``frequency == Weekly``."""
    turn_on: bool
    """When the rule fires, turn the plug on (``True``) or off (``False``)."""

    @staticmethod
    def clock_weekly(hour: int, minute: int, week_day: int, turn_on: bool) -> "ScheduleRule":
        """Fires every day matched by ``week_day`` at ``hour:minute``."""

    @staticmethod
    def clock_once(hour: int, minute: int, turn_on: bool) -> "ScheduleRule":
        """Fires once, the next time the device's clock reaches ``hour:minute``."""

    @staticmethod
    def sunrise_weekly(offset_minutes: int, week_day: int, turn_on: bool) -> "ScheduleRule":
        """Fires every day matched by ``week_day`` at ``offset_minutes`` from sunrise."""

    @staticmethod
    def sunrise_once(offset_minutes: int, turn_on: bool) -> "ScheduleRule":
        """Fires once at the next sunrise plus ``offset_minutes``."""

    @staticmethod
    def sunset_weekly(offset_minutes: int, week_day: int, turn_on: bool) -> "ScheduleRule":
        """Fires every day matched by ``week_day`` at ``offset_minutes`` from sunset."""

    @staticmethod
    def sunset_once(offset_minutes: int, turn_on: bool) -> "ScheduleRule":
        """Fires once at the next sunset plus ``offset_minutes``."""

    def with_enable(self, enable: bool) -> "ScheduleRule":
        """Returns a copy of this rule with ``enable`` set."""

    def with_id(self, id: str) -> "ScheduleRule":
        """Returns a copy of this rule with ``id`` set."""
