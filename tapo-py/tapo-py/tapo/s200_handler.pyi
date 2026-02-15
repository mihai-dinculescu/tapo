from typing import List, Literal, Optional
from tapo.responses import S200Result

class S200Handler:
    """Handler for the [S200B](https://www.tapo.com/en/search/?q=S200B) and
    [S200D](https://www.tapo.com/en/search/?q=S200D) devices."""

    async def get_device_info(self) -> S200Result:
        """Returns *device info* as `S200Result`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `S200Handler.get_device_info_json`.

        Returns:
            S200Result: Device info of Tapo S200B and S200D button switches.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_trigger_logs(self, page_size: int, start_id: int) -> TriggerLogsS200Result:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first)

        Use a `start_id` of `0` to get the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsS200Result: Trigger logs result.
        """

class TriggerLogsS200Result:
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[S200Log]
    """Log items in reverse chronological order (newest first)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class S200Log:
    """S200B and S200D Log."""

    event: Literal["rotation", "singleClick", "doubleClick", "lowBattery"]
    id: int
    timestamp: int
    params: Optional[S200RotationParams]

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class S200RotationParams:
    """S200B and S200D Rotation log params."""

    rotation_degrees: int

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
