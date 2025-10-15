from typing import List, Literal
from tapo.responses import T100Result

class T100Handler:
    """Handler for the [T100](https://www.tapo.com/en/search/?q=T100) devices."""

    async def get_device_info(self) -> T100Result:
        """Returns *device info* as `T100Result`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `T100Handler.get_device_info_json`.

        Returns:
            T100Result: Device info of Tapo T100 motion sensor.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_trigger_logs(self, page_size: int, start_id: int) -> TriggerLogsT100Result:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first)

        Use a `start_id` of `0` to get the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsT100Result: Trigger logs result.
        """

class TriggerLogsT100Result:
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[T100Log]
    """Log items in reverse chronological order (newest first)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class T100Log:
    """T110 Log."""

    event: Literal["motion"]
    id: int
    timestamp: int

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
