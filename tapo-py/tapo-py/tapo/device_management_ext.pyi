from typing import Protocol

from tapo.responses import (
    FirmwareAutoUpdateInfoResult,
    FirmwareUpdateStateResult,
    LatestFirmwareResult,
)

class DeviceManagementExt(Protocol):
    """Extension class for device management capabilities like `device_reboot`, `device_reset` and firmware updates."""

    async def device_reboot(self, delay_s: int) -> None:
        """*Reboots* the device.

        Notes:
            * Using a very small delay (e.g. 0 seconds) may cause a `ConnectionReset` or `TimedOut` error as the device reboots immediately.
            * Using a larger delay (e.g. 2-3 seconds) allows the device to respond before rebooting, reducing the chance of errors.
            * With larger delays, the method completes successfully before the device reboots.
              However, subsequent commands may fail if sent during the reboot process or before the device reconnects to the network.

        Args:
            delay_s (int): The delay in seconds before the device is rebooted.
        """

    async def device_reset(self) -> None:
        """*Hardware resets* the device.

        Warning:
            This action will reset the device to its factory settings.
            The connection to the Wi-Fi network and the Tapo app will be lost,
            and the device will need to be reconfigured.

        This feature is especially useful when the device is difficult to access
        and requires reconfiguration.
        """

    async def get_latest_firmware(self) -> LatestFirmwareResult:
        """Returns the *latest firmware* that the device can update to as `LatestFirmwareResult`.

        The device looks the update up by itself. `need_to_upgrade` is `True` when an
        update is available.

        Returns:
            LatestFirmwareResult: The newest firmware the device can update to.
        """

    async def start_firmware_update(self) -> None:
        """*Starts a firmware update*.

        Call `get_latest_firmware` first, and only start an update when one is available.
        With no update available, the device refuses the request with a `FIRMWARE` device
        error (code `-1401`).

        Notes:
            * The method returns as soon as the device accepts the request. The device then
              downloads the firmware by itself, flashes it and reboots.
            * Follow the progress with `get_firmware_update_state`.
            * Subsequent commands may fail with a `ConnectionReset` or `TimedOut` error while the
              device flashes the firmware and reboots, and until it reconnects to the network.
        """

    async def get_firmware_update_state(self) -> FirmwareUpdateStateResult:
        """Returns the *state of a firmware update* as `FirmwareUpdateStateResult`.

        Returns:
            FirmwareUpdateStateResult: The progress of the firmware update.
        """

    async def get_firmware_auto_update_info(self) -> FirmwareAutoUpdateInfoResult:
        """Returns the *automatic firmware update settings* as `FirmwareAutoUpdateInfoResult`.

        Returns:
            FirmwareAutoUpdateInfoResult: The device's automatic firmware update settings.
        """

    async def set_firmware_auto_update_info(self, enabled: bool, start_min: int) -> None:
        """*Sets the automatic firmware update settings*.

        Like the Tapo app, it always sets a two-hour update window that starts on the
        hour. The device expects all its settings at once, so to change only one of
        them, read the others with `get_firmware_auto_update_info` first.

        Args:
            enabled (bool): Whether the device installs firmware updates automatically.
            start_min (int): Start of the update window, in minutes after midnight in the
                device's timezone (`120` is 2:00 AM). Must be a whole hour: `0`, `60`, ... `1380`.
        """
