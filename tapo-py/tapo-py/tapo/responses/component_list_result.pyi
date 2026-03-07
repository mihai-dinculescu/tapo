from tapo.to_dict_ext import ToDictExt

class Component(ToDictExt):
    """A component (feature/capability) reported by a Tapo device."""

    id: str
    """The component identifier (e.g. ``"energy_monitoring"``, ``"countdown"``)."""

    ver_code: int
    """The version code of the component."""
