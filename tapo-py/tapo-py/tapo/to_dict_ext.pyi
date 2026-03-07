from typing import Protocol

class ToDictExt(Protocol):
    """Extension class that provides `to_dict` for converting to a Python dictionary."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
