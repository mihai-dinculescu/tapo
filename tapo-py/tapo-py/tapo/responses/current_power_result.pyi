class CurrentPowerResult:
    """Contains the current power reading of the device."""

    current_power: int
    """Current power in Watts (W)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
