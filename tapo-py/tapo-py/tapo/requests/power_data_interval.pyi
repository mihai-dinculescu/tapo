from typing import Final

class PowerDataInterval:
    """Power data interval."""

    Every5Minutes: Final[PowerDataInterval]
    """Every 5 minutes interval. `start_date_time` and `end_date_time` describe an exclusive interval.
    If the result would yield more than 144 entries (i.e. 12 hours),
    the `end_date_time` will be adjusted to an earlier date and time.
    """

    Hourly: Final[PowerDataInterval]
    """Hourly interval. `start_date_time` and `end_date_time` describe an exclusive interval.
    If the result would yield more than 144 entries (i.e. 6 days),
    the `end_date_time` will be adjusted to an earlier date and time.
    """
