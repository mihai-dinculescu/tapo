from typing import Type
from dataclasses import dataclass
from tapo.to_dict_ext import ToDictExt

@dataclass
class LastStates:
    __match_args__: tuple[str, ...] = ()

@dataclass
class Custom:
    state: PlugState

    __match_args__ = ("state",)

class DefaultPlugState(ToDictExt):
    """Plug Default State."""

    LastStates: Type[LastStates] = LastStates
    Custom: Type[Custom] = Custom

class PlugState(ToDictExt):
    """Plug State."""

    on: bool
