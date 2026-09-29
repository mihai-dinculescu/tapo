from datetime import datetime
from typing import Final, List

from tapo.responses.child_device_list_hub_result.hub_result import HubResultBase
from tapo.to_dict_ext import ToDictExt

class T300Result(HubResultBase):
    """Device info of Tapo T300 water sensor.

    Specific properties: `in_alarm`, `water_leak_status`, `report_interval`,
    `last_onboarding_timestamp`, `status_follow_edge`.
    """

    in_alarm: bool
    last_onboarding_timestamp: int
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool
    water_leak_status: WaterLeakStatus

class WaterLeakStatus:
    """Water leak status."""

    Normal: Final[WaterLeakStatus]
    WaterDry: Final[WaterLeakStatus]
    WaterLeak: Final[WaterLeakStatus]

class TriggerLogsT300Result(ToDictExt):
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[T300Log]
    """Log items in reverse chronological order (newest first)."""

class T300Log(ToDictExt):
    """One entry in the trigger log of a T300."""

    id: int
    """Hub-assigned id of the log item. The hub numbers the logs of all its sensors from one counter, so newer items have larger ids."""
    triggered_at: datetime
    """When the event happened, in UTC."""
    event: T300Event
    """What happened."""

class T300Event:
    """T300 trigger log event."""

    WaterDry: Final[T300Event]
    WaterLeak: Final[T300Event]
