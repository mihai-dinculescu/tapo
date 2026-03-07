from enum import Enum
from tapo.to_dict_ext import ToDictExt

class DefaultBrightnessState(ToDictExt):
    """Default brightness state."""

    type: DefaultStateType
    value: int

class DefaultStateType(str, Enum):
    """The type of the default state."""

    Custom = "custom"
    LastStates = "last_states"

class DefaultPowerType(str, Enum):
    """The type of the default power state."""

    AlwaysOn = "always_on"
    LastStates = "last_states"
