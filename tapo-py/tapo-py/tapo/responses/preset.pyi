from tapo.to_dict_ext import ToDictExt

class Preset(ToDictExt):
    """A single PTZ preset position."""

    id: str
    """Preset identifier."""

    name: str
    """User-assigned name."""

    pan: float
    """Pan position (normalized, typically -1.0 to 1.0)."""

    tilt: float
    """Tilt position (normalized, typically -1.0 to 1.0)."""

    read_only: bool
    """Whether this preset is read-only."""
