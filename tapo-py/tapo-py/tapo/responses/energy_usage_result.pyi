from datetime import datetime

class EnergyUsageResult:
    """Contains local time, current power and the energy usage and runtime for today and for the current month."""

    local_time: datetime
    """Local time of the device."""
    current_power: int
    """Current power in milliwatts (mW)."""
    today_runtime: int
    """Today runtime in minutes."""
    today_energy: int
    """Today energy usage in watts (W)."""
    month_runtime: int
    """Current month runtime in minutes."""
    month_energy: int
    """Current month energy usage in watts (W)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
