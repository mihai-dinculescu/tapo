from enum import Enum

from tapo.responses.child_device_list_hub_result.hub_result import HubResult

class KE100Result(HubResult):
    """Device info of Tapo KE100 thermostatic radiator valve (TRV).

    Specific properties: `temperature_unit`, `current_temperature`, `target_temperature`,
    `min_control_temperature`, `max_control_temperature`, `temperature_offset`,
    `child_protection_on`, `frost_protection_on`, `location`.
    """

    child_protection_on: bool
    current_temperature: float
    frost_protection_on: bool
    location: str
    max_control_temperature: int
    min_control_temperature: int
    target_temperature: float
    temperature_offset: int
    temperature_unit: TemperatureUnitKE100

class TemperatureUnitKE100(str, Enum):
    """Temperature unit for KE100 devices.
    Currently *Celsius* is the only unit supported by KE100.
    """

    Celsius = "Celsius"
