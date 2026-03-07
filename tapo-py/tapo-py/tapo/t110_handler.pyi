from typing import List, Literal

from tapo.debug_ext import DebugExt
from tapo.responses import T110Result
from tapo.to_dict_ext import ToDictExt

class T110Handler(DebugExt):
    """Handler for the [T110](https://www.tapo.com/en/search/?q=T110) devices."""

    async def get_device_info(self) -> T110Result:
        """Returns *device info* as `T110Result`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `T110Handler.get_device_info_json`.

        Returns:
            T110Result: Device info of Tapo T110 contact sensor.
        """

    async def get_trigger_logs(self, page_size: int, start_id: int) -> TriggerLogsT110Result:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first)

        Use a `start_id` of `0` to get the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsT110Result: Trigger logs result.
        """

class TriggerLogsT110Result(ToDictExt):
    """Trigger logs result."""

    start_id: int
    """The `id` of the most recent log item that is returned."""
    sum: int
    """The total number of log items that the hub holds for this device."""
    logs: List[T110Log]
    """Log items in reverse chronological order (newest first)."""

class T110Log(ToDictExt):
    """T110 Log."""

    event: Literal["close", "open", "keepOpen"]
    id: int
    timestamp: int
