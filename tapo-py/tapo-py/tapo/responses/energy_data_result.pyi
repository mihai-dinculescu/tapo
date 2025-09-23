from datetime import datetime
from typing import List, Optional

class EnergyDataResult:
    """Energy data result for the requested `EnergyDataInterval`."""

    local_time: datetime
    """Local time of the device."""

    start_date_time: datetime
    """Start date and time of this result in UTC.
    This value is provided in the `get_energy_data` request and is passed through.
    Note that it may not align with the returned data if the method is used beyond its specified capabilities."""

    entries: List[EnergyDataIntervalResult]
    """List of energy data entries."""

    interval_length: int
    """Interval length in minutes."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class EnergyDataIntervalResult:
    """Energy data result for a specific interval."""

    start_date_time: datetime
    """Start date and time of this interval in UTC."""

    energy: Optional[int]
    """Energy in Watt Hours (Wh)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
