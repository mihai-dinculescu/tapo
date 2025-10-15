from tapo.requests import TemperatureUnitKE100
from tapo.responses import KE100Result

class KE100Handler:
    """Handler for the [KE100](https://www.tp-link.com/en/search/?q=KE100) devices."""

    async def get_device_info(self) -> KE100Result:
        """Returns *device info* as `KE100Result`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `KE100Handler.get_device_info_json`.

        Returns:
            KE100Result: Device info of Tapo KE100 thermostatic radiator valve (TRV).
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def set_child_protection(self, on: bool) -> None:
        """Sets *child protection* on the device to *on* or *off*.

        Args:
            on (bool)
        """

    async def set_frost_protection(self, on: bool) -> None:
        """Sets *frost protection* on the device to *on* or *off*.

        Args:
            on (bool)
        """

    async def set_max_control_temperature(self, value: int, unit: TemperatureUnitKE100) -> None:
        """Sets the *maximum control temperature*.

        Args:
            value (int)
            unit (TemperatureUnitKE100)
        """

    async def set_min_control_temperature(self, value: int, unit: TemperatureUnitKE100) -> None:
        """Sets the *minimum control temperature*.

        Args:
            value (int)
            unit (TemperatureUnitKE100)
        """

    async def set_target_temperature(self, value: int, unit: TemperatureUnitKE100) -> None:
        """Sets the *target temperature*.

        Args:
            value (int): between `min_control_temperature` and `max_control_temperature`
            unit (TemperatureUnitKE100)
        """

    async def set_temperature_offset(self, value: int, unit: TemperatureUnitKE100) -> None:
        """Sets the *temperature offset*.

        Args:
            value (int): between -10 and 10
            unit (TemperatureUnitKE100)
        """
