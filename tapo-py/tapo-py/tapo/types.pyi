from enum import StrEnum

class DefaultBrightnessState:
    """Default brightness state."""

    type: DefaultStateType
    value: int

class DefaultStateType(StrEnum):
    """The type of the default state."""

    Custom = "custom"
    LastStates = "last_states"

class DefaultPowerType(StrEnum):
    """The type of the default power state."""

    AlwaysOn = "always_on"
    LastStates = "last_states"

class UsageByPeriodResult:
    """Usage by period result for today, the past 7 days, and the past 30 days."""

    today: int
    """Today."""
    past7: int
    """Past 7 days."""
    past30: int
    """Past 30 days."""

class DeviceUsageResult:
    """Contains the time in use, the power consumption, and the energy savings of the device."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""
    power_usage: UsageByPeriodResult
    """Power usage in watt-hour (Wh)."""
    saved_power: UsageByPeriodResult
    """Saved power in watt-hour (Wh)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DeviceUsageEnergyMonitoringResult:
    """Contains the time in use, the power consumption, and the energy savings of the device."""

    time_usage: UsageByPeriodResult
    """Time usage in minutes."""
    power_usage: UsageByPeriodResult
    """Power usage in watt-hour (Wh)."""
    saved_power: UsageByPeriodResult
    """Saved power in watt-hour (Wh)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class Color(StrEnum):
    """List of preset colors as defined in the Google Home app."""

    CoolWhite = "CoolWhite"
    Daylight = "Daylight"
    Ivory = "Ivory"
    WarmWhite = "WarmWhite"
    Incandescent = "Incandescent"
    Candlelight = "Candlelight"
    Snow = "Snow"
    GhostWhite = "GhostWhite"
    AliceBlue = "AliceBlue"
    LightGoldenrod = "LightGoldenrod"
    LemonChiffon = "LemonChiffon"
    AntiqueWhite = "AntiqueWhite"
    Gold = "Gold"
    Peru = "Peru"
    Chocolate = "Chocolate"
    SandyBrown = "SandyBrown"
    Coral = "Coral"
    Pumpkin = "Pumpkin"
    Tomato = "Tomato"
    Vermilion = "Vermilion"
    OrangeRed = "OrangeRed"
    Pink = "Pink"
    Crimson = "Crimson"
    DarkRed = "DarkRed"
    HotPink = "HotPink"
    Smitten = "Smitten"
    MediumPurple = "MediumPurple"
    BlueViolet = "BlueViolet"
    Indigo = "Indigo"
    LightSkyBlue = "LightSkyBlue"
    CornflowerBlue = "CornflowerBlue"
    Ultramarine = "Ultramarine"
    DeepSkyBlue = "DeepSkyBlue"
    Azure = "Azure"
    NavyBlue = "NavyBlue"
    LightTurquoise = "LightTurquoise"
    Aquamarine = "Aquamarine"
    Turquoise = "Turquoise"
    LightGreen = "LightGreen"
    Lime = "Lime"
    ForestGreen = "ForestGreen"
