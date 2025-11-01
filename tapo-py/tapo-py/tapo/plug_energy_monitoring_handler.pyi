from datetime import datetime

from tapo.device_management_ext import DeviceManagementExt
from tapo.requests import EnergyDataInterval, PowerDataInterval
from tapo.responses import (
    CurrentPowerResult,
    DeviceInfoPlugEnergyMonitoringResult,
    DeviceUsageEnergyMonitoringResult,
    EnergyDataResult,
    EnergyUsageResult,
    PowerDataResult,
)

class PlugEnergyMonitoringHandler(DeviceManagementExt):
    """Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
    [P110M](https://www.tapo.com/en/search/?q=P110M) and
    [P115](https://www.tapo.com/en/search/?q=P115) devices.
    """

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

    async def get_device_info(self) -> DeviceInfoPlugEnergyMonitoringResult:
        """Returns *device info* as `DeviceInfoPlugEnergyMonitoringResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `PlugEnergyMonitoringHandler.get_device_info_json`.

        Returns:
            DeviceInfoPlugEnergyMonitoringResult: Device info of P110, P110M and P115.
            Superset of `GenericDeviceInfoResult`.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_current_power(self) -> CurrentPowerResult:
        """Returns *current power* as `CurrentPowerResult`.

        Returns:
            CurrentPowerResult: Contains the current power reading of the device.
        """

    async def get_device_usage(self) -> DeviceUsageEnergyMonitoringResult:
        """Returns *device usage* as `DeviceUsageResult`.

        Returns:
            DeviceUsageEnergyMonitoringResult:
            Contains the time usage, the power consumption, and the energy savings of the device.
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
            EnergyDataResult: Energy data result for the requested `EnergyDataInterval`.
        """

    async def get_power_data(
        self,
        interval: PowerDataInterval,
        start_date_time: datetime,
        end_date_time: datetime,
    ) -> PowerDataResult:
        """Returns *power data* as `PowerDataResult`.

        Returns:
            PowerDataResult: Power data result for the requested `PowerDataInterval`.
        """
