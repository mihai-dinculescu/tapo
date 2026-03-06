class Component:
    """A component (feature/capability) reported by a Tapo device."""

    id: str
    """The component identifier (e.g. ``"energy_monitoring"``, ``"countdown"``)."""

    ver_code: int
    """The version code of the component."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
