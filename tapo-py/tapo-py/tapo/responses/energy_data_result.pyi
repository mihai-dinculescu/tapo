from datetime import datetime
from typing import List

class EnergyDataResult:
    """Energy data for the requested `EnergyDataInterval`."""

    local_time: datetime
    """Local time of the device."""

    data: List[int]
    """Energy data for the given `interval` in watts (W)."""

    start_timestamp: int
    """Interval start timestamp in milliseconds."""

    end_timestamp: int
    """Interval end timestamp in milliseconds."""

    interval: int
    """Interval in minutes."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
