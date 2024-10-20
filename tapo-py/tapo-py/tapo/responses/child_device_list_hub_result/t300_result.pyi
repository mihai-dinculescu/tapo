from enum import Enum
from tapo.responses.child_device_list_hub_result.hub_result import HubResult

class T300Result(HubResult):
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

class WaterLeakStatus(str, Enum):
    """Water leak status."""

    Normal = "Normal"
    WaterDry = "WaterDry"
    WaterLeak = "WaterLeak"
