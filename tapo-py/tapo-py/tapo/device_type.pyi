from typing import Final

class DeviceType:
    """Categorizes a Tapo device by its capabilities."""

    Other: Final[DeviceType]
    """A Tapo device without a specific handler implementation."""

    Light: Final[DeviceType]
    """Tapo L510, L520, L610 - dimmable lights."""

    ColorLight: Final[DeviceType]
    """Tapo L530, L535, L630 - color lights."""

    RgbLightStrip: Final[DeviceType]
    """Tapo L900 - RGB light strip."""

    RgbicLightStrip: Final[DeviceType]
    """Tapo L920, L930 - RGBIC light strip."""

    Plug: Final[DeviceType]
    """Tapo P100, P105, P125, P125M - smart plugs."""

    PlugEnergyMonitoring: Final[DeviceType]
    """Tapo P110, P110M, P115 - smart plugs with energy monitoring."""

    PowerStrip: Final[DeviceType]
    """Tapo P300, P306 - power strips."""

    PowerStripEnergyMonitoring: Final[DeviceType]
    """Tapo P304M, P316M - power strips with energy monitoring."""

    Hub: Final[DeviceType]
    """Tapo H100 - smart hub."""

    CameraHub: Final[DeviceType]
    """Tapo H200, H500 - camera hubs."""

    CameraPtz: Final[DeviceType]
    """Tapo C210, C220, C225, C325WB, C520WS, TC40, TC70 - smart cameras with PTZ."""
