from zoneinfo import ZoneInfo

from tapo.to_dict_ext import ToDictExt

class TimezoneResult(ToDictExt):
    """The timezone configured on a device."""

    timezone: ZoneInfo
    """The device's timezone, from the IANA name it reports, e.g. `Europe/London`."""

    standard_offset: int
    """The device's standard UTC offset in minutes.
    It does not follow daylight saving, so it can be an hour off the
    offset in effect; convert times with `timezone` instead."""
