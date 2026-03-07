from typing import Protocol

class OnOffExt(Protocol):
    """Extension class for on/off capabilities."""

    async def on(self) -> None:
        """Turns *on* the device."""

    async def off(self) -> None:
        """Turns *off* the device."""
