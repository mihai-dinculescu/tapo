from tapo.debug_ext import DebugExt
from tapo.responses import S200Result, TriggerLogsS200Result

class S200Handler(DebugExt):
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

    async def get_trigger_logs(self, page_size: int, start_id: int = 0) -> TriggerLogsS200Result:
        """Returns a list of *trigger logs*.

        Args:
            page_size (int): the maximum number of log items to return
            start_id (int): the log item `id` from which to start returning results
                in reverse chronological order (newest first). Defaults to `0`,
                which returns the most recent X logs, where X is capped by `page_size`.

        Returns:
            TriggerLogsS200Result: Trigger logs result.
        """
