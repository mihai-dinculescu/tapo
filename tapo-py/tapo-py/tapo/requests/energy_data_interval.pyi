from enum import Enum

class EnergyDataInterval(str, Enum):
    """Energy data interval."""

    Hourly = "Hourly"
    """Hourly interval. `start_date` and `end_date` are an inclusive interval
    that must not be greater than 8 days.
    """

    Daily = "Daily"
    """Daily interval. `start_date` must be the first day of a quarter."""

    Monthly = "Monthly"
    """Monthly interval. `start_date` must be the first day of a year."""
