from enum import Enum

class DefaultBrightnessState:
    """Default brightness state."""

    type: DefaultStateType
    value: int

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class DefaultStateType(str, Enum):
    """The type of the default state."""

    Custom = "custom"
    LastStates = "last_states"

class DefaultPowerType(str, Enum):
    """The type of the default power state."""

    AlwaysOn = "always_on"
    LastStates = "last_states"
