from zoneinfo import ZoneInfo

from tapo.to_dict_ext import ToDictExt

class TimezoneResult(ToDictExt):
    """The timezone configured on a device."""

    timezone: ZoneInfo
    """The device's timezone, from the IANA name it reports, e.g. `Europe/London`."""

    standard_offset: int
    """The UTC offset in minutes that the device reports.
    It is usually the standard offset, without daylight saving, but apps
    that set the device's time can leave the offset in effect at that
    moment, or a stale one. Convert times with `timezone` instead."""
