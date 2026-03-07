from datetime import datetime
from typing import Optional, Tuple
from tapo.to_dict_ext import ToDictExt

class EnergyUsageResult(ToDictExt):
    """Contains local time, current power and the energy usage and runtime for today and for the current month."""

    current_power: Optional[int]
    """Current power in milliwatts (mW)."""
    electricity_charge: Optional[Tuple[int, int, int]]
    """Electricity charge/cost data reported by the device using the tariff configured in the Tapo app.
    The third element is the total charge for the current month.
    The meaning of the first two elements is not confirmed; please open an issue or pull request if you know."""
    local_time: datetime
    """Local time of the device."""
    month_energy: int
    """Current month energy usage in Watt Hours (Wh)."""
    month_runtime: int
    """Current month runtime in minutes."""
    today_energy: int
    """Today energy usage in Watt Hours (Wh)."""
    today_runtime: int
    """Today runtime in minutes."""
