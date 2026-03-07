from .component_list_result import Component
from tapo.to_dict_ext import ToDictExt

class ChildDeviceComponentList(ToDictExt):
    """A single child device's component (feature/capability) list."""

    device_id: str
    """The device ID of the child device."""

    component_list: list[Component]
    """The list of components supported by this child device."""
