from datetime import datetime
from enum import StrEnum
from typing import List

from .types import DeviceUsageEnergyMonitoringResult
from .plug_handler import DeviceInfoPlugResult

class PlugEnergyMonitoringHandler:
    """Handler for the [P110](https://www.tapo.com/en/search/?q=P110) & [P115](https://www.tapo.com/en/search/?q=P115) devices."""

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """
    async def refresh_session(self) -> None:
        """Refreshes the authentication session."""
    async def on(self) -> None:
        """Turns *on* the device."""
    async def off(self) -> None:
        """Turns *off* the device."""
    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """
    async def get_device_info(self) -> DeviceInfoPlugResult:
        """Returns *device info* as `DeviceInfoPlugResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PlugEnergyMonitoringHandler.get_device_info_json`.

        Returns:
            DeviceInfoPlugResult: Device info of Tapo P100, P105, P110 and P115.
            Superset of `GenericDeviceInfoResult`.
        """
    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """
    async def get_device_usage(self) -> DeviceUsageEnergyMonitoringResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageEnergyMonitoringResult:
            Contains the time usage, the power consumption, and the energy savings of the device.
        """
    async def get_current_power(self) -> CurrentPowerResult:
        """Returns *current power* as `CurrentPowerResult`.

        Returns:
            CurrentPowerResult: Contains the current power reading of the device.
        """
    async def get_energy_usage(self) -> EnergyUsageResult:
        """Returns *energy usage* as `EnergyUsageResult`.

        Returns:
            EnergyUsageResult:
            Contains local time, current power and the energy usage and runtime for today and for the current month.
        """
    async def get_energy_data(
        self,
        interval: EnergyDataInterval,
        start_date: datetime,
        end_date: datetime = None,
    ) -> EnergyDataResult:
        """Returns *energy data* as `EnergyDataResult`.

        Returns:
            EnergyDataResult: Energy data for the requested `EnergyDataInterval`.
        """

class CurrentPowerResult:
    """Contains the current power reading of the device."""

    current_power: int
    """Current power in watts (W)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class EnergyUsageResult:
    """Contains local time, current power and the energy usage and runtime for today and for the current month."""

    local_time: datetime
    """Local time of the device."""
    current_power: int
    """Current power in milliwatts (mW)."""
    today_runtime: int
    """Today runtime in minutes."""
    today_energy: int
    """Today energy usage in watts (W)."""
    month_runtime: int
    """Current month runtime in minutes."""
    month_energy: int
    """Current month energy usage in watts (W)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class EnergyDataResult:
    """Energy data for the requested `EnergyDataInterval`."""

    local_time: datetime
    """Local time of the device."""

    data: List[int]
    """Energy data for the given `interval` in watts (W)."""

    start_timestamp: int
    """Interval start timestamp in milliseconds."""

    end_timestamp: int
    """Interval end timestamp in milliseconds."""

    interval: int
    """Interval in minutes."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class EnergyDataInterval(StrEnum):
    """Energy data interval."""

    Hourly = "Hourly"
    """Hourly interval. `start_date` and `end_date` are an inclusive interval
    that must not be greater than 8 days.
    """

    Daily = "Daily"
    """Daily interval. `start_date` must be the first day of a quarter."""

    Monthly = "Monthly"
    """Monthly interval. `start_date` must be the first day of a year."""
