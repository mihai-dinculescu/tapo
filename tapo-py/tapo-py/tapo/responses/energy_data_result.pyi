from datetime import datetime
from typing import List, Optional
from tapo.to_dict_ext import ToDictExt

class EnergyDataResult(ToDictExt):
    """Energy data result for the requested `EnergyDataInterval`."""

    local_time: datetime
    """Local time of the device."""

    start_date_time: datetime
    """Start date and time of this result in UTC.
    This value is provided in the `get_energy_data` request and is passed through.
    Note that it may not align with the returned data if the method is used beyond its specified capabilities."""

    entries: List[EnergyDataIntervalResult]
    """List of energy data entries, up to the device's local time. It is shorter
    than the requested interval when the interval has not ended yet, and empty
    when it has not started.

    Hourly entries come 24 to a local day. The hour that the clocks go back
    over is a single entry, covering both times it happens. The hour that the
    clocks jump forward over has an entry too, with the same start as the hour
    after it."""

    interval_length: int
    """Interval length in minutes."""

class EnergyDataIntervalResult(ToDictExt):
    """Energy data result for a specific interval."""

    start_date_time: datetime
    """Start date and time of this interval in UTC."""

    energy: Optional[int]
    """Energy in Watt Hours (Wh)."""
