from enum import Enum

class Status(str, Enum):
    """Device status."""

    Online = "Online"
    Offline = "Offline"
