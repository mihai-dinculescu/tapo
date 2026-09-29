from datetime import datetime
from typing import Final, List

from tapo.responses.child_device_list_hub_result.hub_result import HubResultBase
from tapo.to_dict_ext import ToDictExt

class T110Result(HubResultBase):
    """Device info of Tapo T110 contact sensor.

    Specific properties: `open`, `report_interval`,
    `last_onboarding_timestamp`,`status_follow_edge`.
    """

    last_onboarding_timestamp: int
    open: bool
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool

class TriggerLogsT110Result(ToDictExt):
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[T110Log]
    """Log items in reverse chronological order (newest first)."""

class T110Log(ToDictExt):
    """One entry in the trigger log of a T110."""

    id: int
    """Hub-assigned id of the log item. The hub numbers the logs of all its sensors from one counter, so newer items have larger ids."""
    triggered_at: datetime
    """When the event happened, in UTC."""
    event: T110Event
    """What happened."""

class T110Event:
    """T110 trigger log event."""

    Close: Final[T110Event]
    Open: Final[T110Event]
    KeepOpen: Final[T110Event]
    """Fired when the sensor has been open for more than 1 minute."""
