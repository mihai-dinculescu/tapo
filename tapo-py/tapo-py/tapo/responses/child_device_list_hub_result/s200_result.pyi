from datetime import datetime
from typing import Final, List, Optional

from tapo.responses.child_device_list_hub_result.hub_result import HubResultBase
from tapo.to_dict_ext import ToDictExt

class S200Result(HubResultBase):
    """Device info of Tapo S200B and S200D button switches.

    Specific properties: `report_interval`, `last_onboarding_timestamp`, `status_follow_edge`.
    """

    last_onboarding_timestamp: int
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool

class TriggerLogsS200Result(ToDictExt):
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[S200Log]
    """Log items in reverse chronological order (newest first)."""

class S200Log(ToDictExt):
    """One entry in the trigger log of a S200B or S200D."""

    id: int
    """Hub-assigned id of the log item. The hub numbers the logs of all its sensors from one counter, so newer items have larger ids."""
    triggered_at: datetime
    """When the event happened, in UTC."""
    event: S200Event
    """What happened."""
    params: Optional[S200RotationParams]
    """The rotation details when `event` is `S200Event.Rotation`, `None` otherwise."""

class S200Event:
    """S200B and S200D trigger log event."""

    Rotation: Final[S200Event]
    SingleClick: Final[S200Event]
    DoubleClick: Final[S200Event]
    LowBattery: Final[S200Event]

class S200RotationParams(ToDictExt):
    """S200B and S200D Rotation log params."""

    rotation_degrees: int
