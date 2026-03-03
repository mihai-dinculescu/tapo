import enum

class DeviceType(enum.Enum):
    """Categorizes a Tapo device by its capabilities."""

    GenericDevice = ...
    """A generic/unknown Tapo device."""

    Light = ...
    """Tapo L510, L520, L610 — dimmable lights."""

    ColorLight = ...
    """Tapo L530, L535, L630 — color lights."""

    RgbLightStrip = ...
    """Tapo L900 — RGB light strip."""

    RgbicLightStrip = ...
    """Tapo L920, L930 — RGBIC light strip."""

    Plug = ...
    """Tapo P100, P105 — smart plugs."""

    PlugEnergyMonitoring = ...
    """Tapo P110, P110M, P115 — smart plugs with energy monitoring."""

    PowerStrip = ...
    """Tapo P300, P306 — power strips."""

    PowerStripEnergyMonitoring = ...
    """Tapo P304M, P316M — power strips with energy monitoring."""

    Hub = ...
    """Tapo H100 — smart hub."""
