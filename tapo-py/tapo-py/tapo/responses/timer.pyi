from enum import Enum

from tapo.to_dict_ext import ToDictExt

class PowerState(str, Enum):
    """The state a plug transitions to when its timer fires."""

    On = "on"
    Off = "off"

class Timer(ToDictExt):
    """A pending "Timer" countdown on a Tapo plug."""

    id: str
    """Device-assigned id."""
    delay_s: int
    """Total countdown duration in seconds."""
    remaining_s: int
    """Seconds left until the timer fires."""
    desired_state: PowerState
    """The state the plug transitions to when the timer fires."""
