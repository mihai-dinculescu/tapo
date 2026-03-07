from tapo.responses import UsageByPeriodResult
from tapo.to_dict_ext import ToDictExt

class DeviceUsageEnergyMonitoringResult(ToDictExt):
    """Contains the time usage, the power consumption, and the energy savings of the device."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""
    power_usage: UsageByPeriodResult
    """Power usage in Watt Hours (Wh)."""
    saved_power: UsageByPeriodResult
    """Saved power in Watt Hours (Wh)."""
