from tapo.responses.child_device_list_hub_result.hub_result import HubResult

class T110Result(HubResult):
    """Device info of Tapo T110 contact sensor.

    Specific properties: `open`, `report_interval`,
    `last_onboarding_timestamp`,`status_follow_edge`.
    """

    last_onboarding_timestamp: int
    open: bool
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool
