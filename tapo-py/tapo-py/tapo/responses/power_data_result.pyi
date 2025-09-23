from datetime import datetime
from typing import List, Optional

class PowerDataResult:
    """Power data result for the requested `PowerDataInterval`."""

    start_date_time: datetime
    """Start date and time of this result in UTC."""

    end_date_time: datetime
    """End date and time of this result in UTC."""

    entries: List[PowerDataIntervalResult]
    """List of power data entries."""

    interval_length: int
    """Interval length in minutes."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class PowerDataIntervalResult:
    """Power data result for a specific interval."""

    start_date_time: datetime
    """Start date and time of this interval in UTC."""

    power: Optional[int]
    """Power in Watts (W). `None` if no data is available for this interval."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
