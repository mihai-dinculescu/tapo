from tapo.responses.child_device_list_hub_result.hub_result import HubResult

class T100Result(HubResult):
    """Device info of Tapo T100 motion sensor.

    Specific properties: `detected`, `report_interval`,
    `last_onboarding_timestamp`, `status_follow_edge`.
    """

    detected: bool
    last_onboarding_timestamp: int
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool
