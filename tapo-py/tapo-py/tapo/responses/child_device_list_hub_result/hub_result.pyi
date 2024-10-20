from tapo.responses.child_device_list_hub_result.status import Status

class HubResult:
    """Hub result. This is an abstract base class for all hub results."""

    at_low_battery: bool
    avatar: str
    bind_count: int
    category: str
    device_id: str
    fw_ver: str
    hw_id: str
    hw_ver: str
    jamming_rssi: int
    jamming_signal_level: int
    mac: str
    nickname: str
    oem_id: str
    parent_device_id: str
    region: str
    rssi: int
    signal_level: int
    specs: str
    status: Status
    type: str

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
