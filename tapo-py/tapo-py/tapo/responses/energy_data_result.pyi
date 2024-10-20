from datetime import datetime
from typing import List

class EnergyDataResult:
    """Energy data for the requested `EnergyDataInterval`."""

    local_time: datetime
    """Local time of the device."""

    data: List[int]
    """Energy data for the given `interval` in Watt Hours (Wh)."""

    start_timestamp: int
    """Start timestamp of the interval in milliseconds. This value is provided
    in the `get_energy_data` request and is passed through. Note that
    it may not align with the returned data if the method is used
    beyond its specified capabilities.
    """

    end_timestamp: int
    """End timestamp of the interval in milliseconds. This value is provided
    in the `get_energy_data` request and is passed through. Note that
    it may not align with the returned data for intervals other than hourly.
    """

    interval: int
    """Interval in minutes."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
