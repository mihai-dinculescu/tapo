from tapo.responses.child_device_list_hub_result.hub_result import HubResultBase

class S210Result(HubResultBase):
    """Device info of Tapo S210 light switch.

    Specific properties: `battery_percentage`, `device_on`,
    `last_onboarding_timestamp`, `position`, `slot_number`, `status_follow_edge`.
    """

    battery_percentage: int
    device_on: bool
    last_onboarding_timestamp: int
    position: int
    slot_number: int
    status_follow_edge: bool
