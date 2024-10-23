from typing import Optional

class UsageByPeriodResult:
    """Usage by period result for today, the past 7 days, and the past 30 days."""

    today: Optional[int]
    """Today."""
    past7: Optional[int]
    """Past 7 days."""
    past30: Optional[int]
    """Past 30 days."""

class DeviceUsageResult:
    """Contains the time usage."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
