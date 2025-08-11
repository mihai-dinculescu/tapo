from typing import Optional, Union

from tapo.responses.device_info_result.default_plug_state import Custom, LastStates
from tapo.responses.device_info_result.power_status import (
    ChargingStatus,
    OvercurrentStatus,
    OverheatStatus,
    PowerProtectionStatus,
)

class DeviceInfoPlugEnergyMonitoringResult:
    """Device info of Tapo P110, P110M and P115. Superset of `GenericDeviceInfoResult`."""

    device_id: str
    type: str
    model: str
    hw_id: str
    hw_ver: str
    fw_id: str
    fw_ver: str
    oem_id: str
    mac: str
    ip: str
    ssid: str
    signal_level: int
    rssi: int
    specs: str
    lang: str
    device_on: bool
    on_time: int
    """The time in seconds this device has been ON since the last state change (On/Off)."""
    nickname: str
    avatar: str
    has_set_location_info: bool
    region: Optional[str]
    latitude: Optional[float]
    longitude: Optional[float]
    time_diff: Optional[int]

    # Unique to this device
    charging_status: ChargingStatus
    default_states: Union[LastStates, Custom]
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    overcurrent_status: OvercurrentStatus
    overheat_status: Optional[OverheatStatus]
    power_protection_status: PowerProtectionStatus

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
