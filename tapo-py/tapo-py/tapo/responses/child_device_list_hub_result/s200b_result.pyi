from tapo.responses.child_device_list_hub_result.hub_result import HubResult

class S200BResult(HubResult):
    """Device info of Tapo S200B button switch.

    Specific properties: `report_interval`, `last_onboarding_timestamp`, `status_follow_edge`.
    """

    last_onboarding_timestamp: int
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool
