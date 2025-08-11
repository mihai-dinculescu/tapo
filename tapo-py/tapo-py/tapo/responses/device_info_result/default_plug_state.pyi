from typing import Type
from dataclasses import dataclass

@dataclass
class LastStates:
    __match_args__: tuple[str, ...] = ()

@dataclass
class Custom:
    state: PlugState

    __match_args__ = ("state",)

class DefaultPlugState:
    """Plug Default State."""

    LastStates: Type[LastStates] = LastStates
    Custom: Type[Custom] = Custom

class PlugState:
    """Plug State."""

    on: bool

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
