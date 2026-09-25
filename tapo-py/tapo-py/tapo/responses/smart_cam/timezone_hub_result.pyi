from zoneinfo import ZoneInfo

from tapo.to_dict_ext import ToDictExt

class TimezoneHubResult(ToDictExt):
    """The timezone configured on a camera hub."""

    timezone: str
    """The hub's standard UTC offset as a label, e.g. `UTC+01:00`.
    It does not follow daylight saving, so it can be an hour off the
    offset in effect; convert times with `zone_id` instead."""

    zone_id: ZoneInfo
    """The hub's timezone, from the IANA name it reports, e.g. `Europe/Paris`."""
