from typing import Protocol

from tapo.responses import FirmwareUpdateStateResult, LatestFirmwareResult

class FirmwareExt(Protocol):
    """Extension class for firmware capabilities."""

    async def get_latest_firmware(self) -> LatestFirmwareResult:
        """Returns the *latest firmware* that the device can update to as `LatestFirmwareResult`.

        When no update is available, `update_available` is `False` and `fw_ver` is
        the version that the device is running.

        Returns:
            LatestFirmwareResult: The newest firmware that the device can update to.
        """

    async def get_firmware_update_state(self) -> FirmwareUpdateStateResult:
        """Returns the *state of a firmware update* on the device as
        `FirmwareUpdateStateResult`.

        When no update is in progress, `status` is `FirmwareUpdateStatus.Idle`.

        Returns:
            FirmwareUpdateStateResult: The state of the firmware update.
        """

    async def start_firmware_update(self) -> None:
        """*Starts a firmware update* to the version that `get_latest_firmware` returns.

        The device downloads the update, installs it and reboots by itself, and this
        method returns as soon as the device accepts the request. Follow the update with
        `get_firmware_update_state`.

        Requests time out while the device reboots. Once it is back, its session is no
        longer valid and requests fail until `refresh_session` on the device's own handler
        logs in again. Hub child devices are reached through the hub, whose session is
        unaffected, and the Tapo app updates the hub first when the hub has an update of
        its own.
        """
