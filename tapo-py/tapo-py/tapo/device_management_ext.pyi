from typing import Protocol

from tapo.responses import FirmwareUpgradeState, LatestFirmwareResult

class DeviceManagementExt(Protocol):
    """Extension class for device management capabilities like `device_reboot`, `device_reset` and the firmware methods."""

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
        """*Returns* the latest firmware that the device reports as available.

        `need_to_upgrade` says whether that version differs from the `fw_ver`
        in `get_device_info()`.
        """

    async def firmware_upgrade(self) -> None:
        """*Tells* the device to *download and install* the latest firmware.

        Warning:
            The device may reboot while flashing.

        Read the progress of the download and the upgrade with
        `get_firmware_upgrade_state()`.
        """

    async def get_firmware_upgrade_state(self) -> FirmwareUpgradeState:
        """*Returns* the firmware upgrade state: the download and upgrade progress
        of an upgrade running on the device.

        `status` comes from the device and is reported as it arrives.
        """
