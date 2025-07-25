from enum import Enum
from typing import Optional

from tapo.responses.device_info_result.power_status import (
    ChargingStatus,
    OvercurrentStatus,
    OverheatStatus,
    PowerProtectionStatus,
)

class PowerStripPlugResult:
    """P300, P304M and P316M power strip child plugs."""

    auto_off_remain_time: int
    auto_off_status: AutoOffStatus
    avatar: str
    bind_count: int
    category: str
    charging_status: ChargingStatus
    device_id: str
    device_on: bool
    fw_id: str
    fw_ver: str
    has_set_location_info: bool
    hw_id: str
    hw_ver: str
    latitude: Optional[int]
    longitude: Optional[int]
    mac: str
    model: str
    nickname: str
    oem_id: str
    on_time: int
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    original_device_id: str
    overcurrent_status: OvercurrentStatus
    overheat_status: Optional[OverheatStatus]
    position: int
    power_protection_status: PowerProtectionStatus
    region: Optional[str]
    slot_number: int
    status_follow_edge: bool
    type: str

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class AutoOffStatus(str, Enum):
    On = "on"
    Off = "off"
