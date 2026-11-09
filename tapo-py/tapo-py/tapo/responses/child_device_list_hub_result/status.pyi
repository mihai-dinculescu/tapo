from typing import Final

class Status:
    """Connection status of a hub child device, as reported by the hub."""

    Online: Final[Status]
    """The hub considers the child device connected.

    The hub sets this, not the device, and is slow to change it. A battery-powered
    child that stops reporting, for example with its battery removed, can stay
    `Online` for many hours, and the hub keeps returning its last reported state in
    the meantime. `Online` does not mean the device is reachable right now.
    """

    Offline: Final[Status]
    """The hub has marked the child device as disconnected."""
