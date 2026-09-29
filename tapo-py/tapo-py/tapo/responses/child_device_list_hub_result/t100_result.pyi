from datetime import datetime
from typing import Final, List

from tapo.responses.child_device_list_hub_result.hub_result import HubResultBase
from tapo.to_dict_ext import ToDictExt

class T100Result(HubResultBase):
    """Device info of Tapo T100 motion sensor.

    Specific properties: `detected`, `report_interval`,
    `last_onboarding_timestamp`, `status_follow_edge`.
    """

    detected: bool
    last_onboarding_timestamp: int
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool

class TriggerLogsT100Result(ToDictExt):
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[T100Log]
    """Log items in reverse chronological order (newest first)."""

class T100Log(ToDictExt):
    """One entry in the trigger log of a T100."""

    id: int
    """Hub-assigned id of the log item. The hub numbers the logs of all its sensors from one counter, so newer items have larger ids."""
    triggered_at: datetime
    """When the event happened, in UTC."""
    event: T100Event
    """What happened."""

class T100Event:
    """T100 trigger log event."""

    Motion: Final[T100Event]
