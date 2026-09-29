from tapo.debug_ext import DebugExt
from tapo.responses import T110Result, TriggerLogsT110Result

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

    async def get_trigger_logs(self, page_size: int, start_id: int = 0) -> TriggerLogsT110Result:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first). Defaults to `0`,
                which returns the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsT110Result: Trigger logs result.
        """
