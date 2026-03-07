from typing import Protocol

class RefreshSessionExt(Protocol):
    """Extension class for session refresh capabilities."""

    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""
