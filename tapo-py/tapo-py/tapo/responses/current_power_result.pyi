from tapo.to_dict_ext import ToDictExt

class CurrentPowerResult(ToDictExt):
    """Contains the current power reading of the device."""

    current_power: int
    """Current power in Watts (W)."""
