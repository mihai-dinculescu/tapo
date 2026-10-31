from tapo.to_dict_ext import ToDictExt

class EnergyMeterDataResult(ToDictExt):
    """Contains the voltage, current and power measured by the device, and the energy it has used since it last restarted."""

    current_ma: int
    """Current in milliamperes (mA)."""
    energy_mwh: int
    """Energy used since the device last restarted, in milliwatt hours (mWh)."""
    energy_wh: int
    """Energy used since the device last restarted, in Watt Hours (Wh)."""
    power_mw: int
    """Active power in milliwatts (mW)."""
    voltage_mv: int
    """Voltage in millivolts (mV)."""
