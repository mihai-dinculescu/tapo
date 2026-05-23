from tapo.to_dict_ext import ToDictExt

class Timer(ToDictExt):
    """A pending "Timer" countdown on a Tapo plug."""

    id: str
    """Device-assigned id."""
    delay_seconds: int
    """Total countdown duration in seconds."""
    remaining_seconds: int
    """Seconds left until the timer fires."""
    turn_on: bool
    """Whether the timer turns the plug on (``True``) or off (``False``) when it fires."""
