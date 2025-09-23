from enum import Enum

class PowerDataInterval(str, Enum):
    """Power data interval."""

    Every5Minutes = "5min"
    """Every 5 minutes interval. `start_date_time` and `end_date_time` describe an exclusive interval.
    If the result would yield more than 144 entries (i.e. 12 hours),
    the `end_date_time` will be adjusted to an earlier date and time.
    """

    Hourly = "Hourly"
    """Hourly interval. `start_date_time` and `end_date_time` describe an exclusive interval.
    If the result would yield more than 144 entries (i.e. 6 days),
    the `end_date_time` will be adjusted to an earlier date and time.
    """
