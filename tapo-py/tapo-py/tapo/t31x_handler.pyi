from tapo.responses import T31XResult, TemperatureHumidityRecords

class T31XHandler:
    """Handler for the [T310](https://www.tapo.com/en/search/?q=T310)
    and [T315](https://www.tapo.com/en/search/?q=T315) devices."""

    async def get_device_info(self) -> T31XResult:
        """Returns *device info* as `T31XResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `T31XHandler.get_device_info_json`.

        Returns:
            T31XResult: Device info of Tapo T310 and T315 temperature and humidity sensors.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_temperature_humidity_records(self) -> TemperatureHumidityRecords:
        """Returns *temperature and humidity records* from the last 24 hours
        at 15 minute intervals as `TemperatureHumidityRecords`.

        Returns:
            TemperatureHumidityRecords: Temperature and Humidity records
            for the last 24 hours at 15 minute intervals.
        """
