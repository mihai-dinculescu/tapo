from typing import Optional

from tapo.responses import PowerState
from tapo.to_dict_ext import ToDictExt

class DaysOfWeek:
    """The days of the week a weekly ``ScheduleRule`` fires on.

    Combine the individual days with ``|``, or use one of the preset
    groups::

        midweek = DaysOfWeek.MON | DaysOfWeek.WED
        assert midweek.contains(DaysOfWeek.MON)
        assert DaysOfWeek.WEEKEND == DaysOfWeek.SUN | DaysOfWeek.SAT
    """

    NONE: "DaysOfWeek"
    """No days. A weekly rule with no days would never fire, so the
    ``*_weekly`` factories reject it."""
    SUN: "DaysOfWeek"
    """Sunday."""
    MON: "DaysOfWeek"
    """Monday."""
    TUE: "DaysOfWeek"
    """Tuesday."""
    WED: "DaysOfWeek"
    """Wednesday."""
    THU: "DaysOfWeek"
    """Thursday."""
    FRI: "DaysOfWeek"
    """Friday."""
    SAT: "DaysOfWeek"
    """Saturday."""
    WEEKDAYS: "DaysOfWeek"
    """Monday through Friday."""
    WEEKEND: "DaysOfWeek"
    """Saturday and Sunday."""
    EVERY_DAY: "DaysOfWeek"
    """Every day of the week."""

    @staticmethod
    def from_bits_truncate(bits: int) -> "DaysOfWeek":
        """Builds a set from a device bitmask, ignoring any bits above
        Saturday. The inverse of ``bits``.

        Args:
            bits: a device bitmask; bit 0 is Sunday through bit 6, Saturday.
        """

    def bits(self) -> int:
        """Returns the device bitmask for this set: bit 0 is Sunday through
        bit 6, Saturday."""

    def contains(self, other: "DaysOfWeek") -> bool:
        """Returns ``True`` if every day in ``other`` is also in this set."""

    def __or__(self, other: "DaysOfWeek") -> "DaysOfWeek": ...

class ScheduleTime:
    """When a ``ScheduleRule`` fires within a day.

    Each variant carries only the fields that apply to it, so a clock rule
    cannot hold a sunrise offset and vice versa."""

    class Clock:
        """At a wall-clock time, in the device's own timezone."""

        hour: int
        """Hour of the day, 0..=23."""
        minute: int
        """Minute of the hour, 0..=59."""

        def __init__(self, hour: int, minute: int) -> None: ...

    class Sunrise:
        """At an offset from civil sunrise, computed by the device."""

        offset_minutes: int
        """Minutes from sunrise; negative fires before it."""

        def __init__(self, offset_minutes: int) -> None: ...

    class Sunset:
        """At an offset from civil sunset, computed by the device."""

        offset_minutes: int
        """Minutes from sunset; negative fires before it."""

        def __init__(self, offset_minutes: int) -> None: ...

class ScheduleRule(ToDictExt):
    """A plug schedule rule (the "Schedule" feature in the Tapo app).

    Construct one with the factory methods below; each raises for
    out-of-range inputs. The device evaluates the time against its own
    configured timezone; you don't supply a calendar date."""

    id: Optional[str]
    """Device-assigned id. ``None`` when constructed locally."""
    enabled: bool
    """Whether the rule is currently active. Disabled rules are kept on the
    device but do not fire."""
    time: ScheduleTime
    """When the rule fires within a day."""
    days: Optional[DaysOfWeek]
    """The days a weekly rule fires on, or ``None`` when it fires once."""
    desired_state: PowerState
    """The state the plug transitions to when the rule fires."""

    @staticmethod
    def clock_weekly(
        hour: int, minute: int, days: DaysOfWeek, desired_state: PowerState
    ) -> "ScheduleRule":
        """Fires every week, on ``days``, at ``hour:minute``.

        Args:
            hour: hour of the day, 0..=23.
            minute: minute of the hour, 0..=59.
            days: the days to fire on; must not be empty.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def clock_once(hour: int, minute: int, desired_state: PowerState) -> "ScheduleRule":
        """Fires once, the next time the device's clock reaches ``hour:minute``.

        Args:
            hour: hour of the day, 0..=23.
            minute: minute of the hour, 0..=59.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def sunrise_weekly(
        offset_minutes: int, days: DaysOfWeek, desired_state: PowerState
    ) -> "ScheduleRule":
        """Fires every week, on ``days``, at ``offset_minutes`` from sunrise.

        Args:
            offset_minutes: minutes from sunrise, -1440..=1440; negative fires
                before it.
            days: the days to fire on; must not be empty.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def sunrise_once(offset_minutes: int, desired_state: PowerState) -> "ScheduleRule":
        """Fires once, at the next sunrise plus ``offset_minutes``.

        Args:
            offset_minutes: minutes from sunrise, -1440..=1440; negative fires
                before it.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def sunset_weekly(
        offset_minutes: int, days: DaysOfWeek, desired_state: PowerState
    ) -> "ScheduleRule":
        """Fires every week, on ``days``, at ``offset_minutes`` from sunset.

        Args:
            offset_minutes: minutes from sunset, -1440..=1440; negative fires
                before it.
            days: the days to fire on; must not be empty.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def sunset_once(offset_minutes: int, desired_state: PowerState) -> "ScheduleRule":
        """Fires once, at the next sunset plus ``offset_minutes``.

        Args:
            offset_minutes: minutes from sunset, -1440..=1440; negative fires
                before it.
            desired_state: the state the plug transitions to when the rule fires.
        """

    def with_enabled(self, enabled: bool) -> "ScheduleRule":
        """Returns a copy of this rule with ``enabled`` set.

        Args:
            enabled: whether the rule should fire; a disabled rule stays on the
                device without firing.
        """

    def with_id(self, id: str) -> "ScheduleRule":
        """Returns a copy of this rule with ``id`` set.

        Args:
            id: the device-assigned id of the rule to update.
        """
