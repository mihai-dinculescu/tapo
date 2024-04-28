from tapo.responses.child_device_hub_result.hub_result import HubResult
from tapo.responses.child_device_hub_result.temperature_unit import TemperatureUnit

class T31XResult(HubResult):
    """T310/T315 temperature & humidity sensor.

    Specific properties: `current_temperature`, `temperature_unit`,
    `current_temperature_exception`, `current_humidity`, `current_humidity_exception`,
    `report_interval`, `last_onboarding_timestamp`, `status_follow_edge`.
    """

    current_humidity_exception: int
    """
    This value will be `0` when the current humidity is within the comfort zone.
    When the current humidity value falls outside the comfort zone, this value
    will be the difference between the current humidity and the lower or upper bound of the comfort zone.
    """
    current_humidity: int
    current_temperature_exception: int
    """
    This value will be `0.0` when the current temperature is within the comfort zone.
    When the current temperature value falls outside the comfort zone, this value
    will be the difference between the current temperature and the lower or upper bound of the comfort zone.
    """
    current_temperature: int
    last_onboarding_timestamp: int
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool
    temperature_unit: TemperatureUnit
