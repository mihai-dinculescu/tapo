from typing import Optional, Union

from tapo.responses.device_info_result.default_plug_state import Custom, LastStates
from tapo.responses.device_info_result.device_info_ext import DeviceInfoSmartExt
from tapo.responses.device_info_result.power_status import (
    ChargingStatus,
    OvercurrentStatus,
    OverheatStatus,
    PowerProtectionStatus,
)
from tapo.to_dict_ext import ToDictExt

class DeviceInfoPlugEnergyMonitoringResult(DeviceInfoSmartExt, ToDictExt):
    """Device info of Tapo P110, P110M and P115."""

    charging_status: ChargingStatus
    default_states: Union[LastStates, Custom]
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    device_on: bool
    nickname: str
    on_time: int
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    overcurrent_status: OvercurrentStatus
    overheat_status: Optional[OverheatStatus]
    power_protection_status: PowerProtectionStatus
