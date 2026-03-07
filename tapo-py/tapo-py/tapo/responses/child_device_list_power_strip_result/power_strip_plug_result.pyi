from typing import Optional, Union

from tapo.responses.child_device_list_power_strip_result.auto_off_status import AutoOffStatus
from tapo.responses.device_info_result.default_plug_state import Custom, LastStates
from tapo.responses.device_info_result.power_status import OverheatStatus
from tapo.to_dict_ext import ToDictExt

class PowerStripPlugResult(ToDictExt):
    """P300 and P306 power strip child plugs.

    Specific properties: `auto_off_remain_time`, `auto_off_status`,
    `bind_count`, `default_states`, `overheat_status`, `position`, `slot_number`.
    """

    auto_off_remain_time: int
    auto_off_status: AutoOffStatus
    avatar: str
    bind_count: int
    category: str
    default_states: Union[LastStates, Custom]
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
    overheat_status: Optional[OverheatStatus]
    position: int
    region: Optional[str]
    slot_number: int
    status_follow_edge: bool
    type: str
