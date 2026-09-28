from typing import Final

class EnergyDataInterval:
    """Energy data interval."""

    Hourly: Final[EnergyDataInterval]
    """Hourly interval. `start_date` and `end_date` are an inclusive interval
    that must not be greater than 8 days.
    """

    Daily: Final[EnergyDataInterval]
    """Daily interval. `start_date` must be the first day of a quarter."""

    Monthly: Final[EnergyDataInterval]
    """Monthly interval. `start_date` must be the first day of a year."""
