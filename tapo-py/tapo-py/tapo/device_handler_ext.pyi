from typing import Protocol

class DeviceHandlerExt(Protocol):
    """Extension class for the capabilities of a handler that talks to the
    device directly, rather than through a hub or a power strip."""

    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""

    @property
    def address(self) -> tuple[str, int]:
        """Returns the IP address and the port that the handler reaches the
        device at. Pass them to an `ApiClient` method as `f"{ip}:{port}"` to
        reach the device again."""
