class DeviceManagementExt:
    """Extension class for device management capabilities like `device_reboot` and `device_reset`."""

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
