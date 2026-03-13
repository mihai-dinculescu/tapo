from typing import Optional
from tapo.to_dict_ext import ToDictExt

class UsageByPeriodResult(ToDictExt):
    """Usage by period result for today, the past 7 days, and the past 30 days."""

    today: Optional[int]
    """Today usage in minutes."""
    past7: Optional[int]
    """Past 7 days usage in minutes."""
    past30: Optional[int]
    """Past 30 days usage in minutes."""

class DeviceUsageResult(ToDictExt):
    """Contains the time usage."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""
