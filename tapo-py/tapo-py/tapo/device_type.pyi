from typing import Final

class DeviceType:
    """Categorizes a Tapo device by its capabilities."""

    Other: Final[DeviceType]
    """A Tapo device without a specific handler implementation."""

    Light: Final[DeviceType]
    """Tapo L510, L520, L610 - dimmable lights."""

    ColorLight: Final[DeviceType]
    """Tapo L430C, L430P, L530, L530EA, L535, L630 - color lights."""

    RgbLightStrip: Final[DeviceType]
    """Tapo L900 - RGB light strip."""

    RgbicLightStrip: Final[DeviceType]
    """Tapo L920, L930 - RGBIC light strip."""

    Plug: Final[DeviceType]
    """Tapo P100, P105, P125, P125M, S500, S505, TP10, TP15, TS15 - smart plugs."""

    PlugEnergyMonitoring: Final[DeviceType]
    """Tapo P110, P110M, P115, P116M - smart plugs with energy monitoring."""

    PowerStrip: Final[DeviceType]
    """Tapo P300, P306, TP25 - power strips."""

    PowerStripEnergyMonitoring: Final[DeviceType]
    """Tapo P210M, P304M, P316M - power strips with energy monitoring."""

    Hub: Final[DeviceType]
    """Tapo H100 - smart hub."""

    HubIr: Final[DeviceType]
    """Tapo H110 - smart hub with an IR blaster."""

    CameraHub: Final[DeviceType]
    """Tapo H200, H500 - camera hubs."""

    Camera: Final[DeviceType]
    """Tapo C100, C101, C110, C120, C460, C720, TC65 - smart cameras without PTZ."""

    CameraPtz: Final[DeviceType]
    """Tapo C200, C210, C211, C220, C225, C325WB, C520WS, TC40, TC70 - smart cameras with PTZ."""

    def __str__(self) -> str:
        """Returns the name of the device type, such as "Power Strip"."""
