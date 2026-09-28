from typing import Final
from tapo.to_dict_ext import ToDictExt

class DefaultBrightnessState(ToDictExt):
    """Default brightness state."""

    type: DefaultStateType
    value: int

class DefaultStateType:
    """The type of the default state."""

    Custom: Final[DefaultStateType]
    LastStates: Final[DefaultStateType]

class DefaultPowerType:
    """The type of the default power state."""

    AlwaysOn: Final[DefaultPowerType]
    LastStates: Final[DefaultPowerType]
