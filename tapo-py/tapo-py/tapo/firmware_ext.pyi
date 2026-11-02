from typing import Protocol

from tapo.responses import LatestFirmwareResult

class FirmwareExt(Protocol):
    """Extension class for firmware capabilities."""

    async def get_latest_firmware(self) -> LatestFirmwareResult:
        """Returns the *latest firmware* that the device can update to as `LatestFirmwareResult`.

        When no update is available, `update_available` is `False` and `fw_ver` is
        the version that the device is running.

        Returns:
            LatestFirmwareResult: The newest firmware that the device can update to.
        """
