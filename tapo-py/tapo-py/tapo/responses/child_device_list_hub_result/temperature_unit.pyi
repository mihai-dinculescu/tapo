from enum import Enum

class TemperatureUnit(str, Enum):
    """Temperature unit."""

    Celsius = "Celsius"
    Fahrenheit = "Fahrenheit"
