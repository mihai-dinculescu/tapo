from datetime import datetime
from typing import List

from tapo.responses import HubResultBase, TemperatureUnit
from tapo.to_dict_ext import ToDictExt

class T31XResult(HubResultBase):
    """Device info of Tapo T310 and T315 temperature and humidity sensors.

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
    current_temperature_exception: float
    """
    This value will be `0.0` when the current temperature is within the comfort zone.
    When the current temperature value falls outside the comfort zone, this value
    will be the difference between the current temperature and the lower or upper bound of the comfort zone.
    """
    current_temperature: float
    last_onboarding_timestamp: int
    report_interval: int
    """The time in seconds between each report."""
    status_follow_edge: bool
    temperature_unit: TemperatureUnit

class TemperatureHumidityRecords(ToDictExt):
    """Temperature and Humidity records for the last 24 hours at 15 minute intervals."""

    retrieved_at: datetime
    """The time on the hub when the records were retrieved, in UTC.
    The last record covers the 15 minute interval this time falls in,
    unless the hub has no data for that interval yet."""
    records: List[TemperatureHumidityRecord]
    temperature_unit: TemperatureUnit

class TemperatureHumidityRecord(ToDictExt):
    """Temperature and Humidity record as an average over a 15 minute interval."""

    recorded_at: datetime
    """The start of the 15 minute interval this record covers, in UTC."""
    humidity_exception: int
    """This value will be `0` when the current humidity is within the comfort zone.
    When the current humidity value falls outside the comfort zone, this value
    will be the difference between the current humidity and the lower or upper bound of the comfort zone."""
    humidity: int
    temperature_exception: float
    """This value will be `0.0` when the current temperature is within the comfort zone.
    When the current temperature value falls outside the comfort zone, this value
    will be the difference between the current temperature and the lower or upper bound of the comfort zone."""
    temperature: float
