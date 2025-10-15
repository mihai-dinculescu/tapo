from typing import List, Literal
from tapo.responses import T300Result

class T300Handler:
    """Handler for the [T300](https://www.tapo.com/en/search/?q=T300) devices."""

    async def get_device_info(self) -> T300Result:
        """Returns *device info* as `T300Result`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `T300Handler.get_device_info_json`.

        Returns:
            T300Result: Device info of Tapo T300 water sensor.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_trigger_logs(self, page_size: int, start_id: int) -> TriggerLogsT300Result:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first)

        Use a `start_id` of `0` to get the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsT300Result: Trigger logs result.
        """

class TriggerLogsT300Result:
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[T300Log]
    """Log items in reverse chronological order (newest first)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class T300Log:
    """T300 Log."""

    event: Literal["waterDry", "waterLeak"]
    id: int
    timestamp: int

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
