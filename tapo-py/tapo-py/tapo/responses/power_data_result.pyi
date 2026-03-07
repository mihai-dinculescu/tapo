from datetime import datetime
from typing import List, Optional
from tapo.to_dict_ext import ToDictExt

class PowerDataResult(ToDictExt):
    """Power data result for the requested `PowerDataInterval`."""

    start_date_time: datetime
    """Start date and time of this result in UTC."""

    end_date_time: datetime
    """End date and time of this result in UTC."""

    entries: List[PowerDataIntervalResult]
    """List of power data entries."""

    interval_length: int
    """Interval length in minutes."""

class PowerDataIntervalResult(ToDictExt):
    """Power data result for a specific interval."""

    start_date_time: datetime
    """Start date and time of this interval in UTC."""

    power: Optional[int]
    """Power in Watts (W). `None` if no data is available for this interval."""
