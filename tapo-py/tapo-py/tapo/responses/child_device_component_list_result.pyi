from .component_list_result import Component

class ChildDeviceComponentList:
    """A single child device's component (feature/capability) list."""

    device_id: str
    """The device ID of the child device."""

    component_list: list[Component]
    """The list of components supported by this child device."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
