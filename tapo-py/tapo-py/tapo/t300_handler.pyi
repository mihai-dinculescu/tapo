from tapo.debug_ext import DebugExt
from tapo.responses import T300Result, TriggerLogsT300Result

class T300Handler(DebugExt):
    """Handler for the [T300](https://www.tapo.com/en/search/?q=T300) devices."""

    async def get_device_info(self) -> T300Result:
        """Returns *device info* as `T300Result`.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `T300Handler.get_device_info_json`.

        Returns:
            T300Result: Device info of Tapo T300 water sensor.
        """

    async def get_trigger_logs(self, page_size: int, start_id: int = 0) -> TriggerLogsT300Result:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first). Defaults to `0`,
                which returns the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsT300Result: Trigger logs result.
        """
