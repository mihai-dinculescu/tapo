from tapo.responses import PowerState

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
            bits: a device bitmask in the range 0..=255; bit 0 is Sunday
                through bit 6, Saturday. Bits 7 and above are ignored, but the
                value must still fit in a byte — a wider int raises
                ``OverflowError``.
        """

    def bits(self) -> int:
        """Returns the device bitmask for this set: bit 0 is Sunday through
        bit 6, Saturday."""

    def contains(self, other: "DaysOfWeek") -> bool:
        """Returns ``True`` if every day in ``other`` is also in this set."""

    def is_empty(self) -> bool:
        """Returns ``True`` if this set contains no days."""

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

class ScheduleRule:
    """A plug schedule rule to send to the device (the "Schedule" feature in
    the Tapo app).

    Values are valid by construction: the factory methods below are the only
    way to make one, and each raises for out-of-range input. Rules read back
    from the device are the separate ``ScheduleRuleResult``; convert one for
    editing with ``ScheduleRuleResult.to_editable``.

    The device evaluates the time against its own configured timezone; you
    don't supply a calendar date."""

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
            offset_minutes: minutes from sunrise, -360..=360; negative fires
                before it.
            days: the days to fire on; must not be empty.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def sunrise_once(offset_minutes: int, desired_state: PowerState) -> "ScheduleRule":
        """Fires once, at the next sunrise plus ``offset_minutes``.

        Args:
            offset_minutes: minutes from sunrise, -360..=360; negative fires
                before it.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def sunset_weekly(
        offset_minutes: int, days: DaysOfWeek, desired_state: PowerState
    ) -> "ScheduleRule":
        """Fires every week, on ``days``, at ``offset_minutes`` from sunset.

        Args:
            offset_minutes: minutes from sunset, -360..=360; negative fires
                before it.
            days: the days to fire on; must not be empty.
            desired_state: the state the plug transitions to when the rule fires.
        """

    @staticmethod
    def sunset_once(offset_minutes: int, desired_state: PowerState) -> "ScheduleRule":
        """Fires once, at the next sunset plus ``offset_minutes``.

        Args:
            offset_minutes: minutes from sunset, -360..=360; negative fires
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
