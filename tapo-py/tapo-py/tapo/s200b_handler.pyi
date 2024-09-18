from typing import List, Literal, Optional
from tapo.responses import S200BResult

class S200BHandler:
    """Handler for the [S200B](https://www.tapo.com/en/search/?q=S200B) devices."""

    async def get_device_info(self) -> S200BResult:
        """Returns *device info* as `S200BResult`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `S200BHandler.get_device_info_json`.

        Returns:
            S200BResult: Device info of Tapo S200B button switch.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def get_trigger_logs(self, page_size: int, start_id: int) -> TriggerLogsS200BResult:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first)

        Use a `start_id` of `0` to get the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsS200BResult: Trigger logs result.
        """

class TriggerLogsS200BResult:
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[S200BLog]
    """Log items in reverse chronological order (newest first)."""

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class S200BLog:
    """S200B Log."""

    event: Literal["rotation", "singleClick", "doubleClick", "lowBattery"]
    id: int
    timestamp: int
    params: Optional[S200BRotationParams]

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """

class S200BRotationParams:
    """S200B Rotation log params."""

    rotation_degrees: int

    def to_dict(self) -> dict:
        """Gets all the properties of this result as a dictionary.

        Returns:
            dict: The result as a dictionary.
        """
