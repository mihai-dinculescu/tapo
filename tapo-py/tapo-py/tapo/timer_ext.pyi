from typing import Optional, Protocol

from tapo.responses import Timer

class TimerExt(Protocol):
    """Extension class for the plug's countdown timer (the "Timer"
    feature in the Tapo app).  Plugs accept at most one armed timer
    at a time."""

    async def set_timer(self, delay_seconds: int, turn_on: bool) -> Timer:
        """Arms the timer for ``delay_seconds`` seconds, replacing any
        timer that is currently armed.  When it fires, the plug turns
        on (``turn_on=True``) or off (``turn_on=False``)."""

    async def get_timer(self) -> Optional[Timer]:
        """Returns the armed timer, or ``None`` if none is armed."""

    async def clear_timer(self) -> None:
        """Cancels the armed timer (the "Stop" button in the Tapo app)."""
