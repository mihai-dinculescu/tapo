from typing import List, Optional, Tuple
from zoneinfo import ZoneInfo

from tapo.responses import DeviceInfoBasicResult, WifiJoinResult, WifiNetwork, WifiSecurity

class SetupHandler:
    """Handler for setting up a new or factory-reset device. It finds the
    Wi-Fi networks that the device sees and puts the device on one of them,
    without the Tapo app.

    A device such as the [P110](https://www.tapo.com/en/search/?q=P110)
    broadcasts a setup network, named after the device, as in
    `Tapo_Plug_999F`. Join it, and the device is at `192.168.0.1`. Build the
    handler with `ApiClient.setup_over_wifi`.

    A device such as the [P110M](https://www.tapo.com/en/search/?q=P110M)
    is set up over Bluetooth instead. Find it with
    `ApiClient.discover_bluetooth_setup_devices` and build the handler with
    `ApiClient.setup_over_bluetooth`.

    The device takes the `ApiClient` credentials as its login once it joins a
    Wi-Fi network, and only those from then on. They need not be those of a
    TP-Link account, because the device does not check them with TP-Link.

    Example:
        ```python
        client = ApiClient("tapo-username@example.com", "tapo-password")
        setup = await client.setup_over_wifi("192.168.0.1")

        networks = await setup.scan_wifi()
        network = next(network for network in networks if network.ssid == "My Wi-Fi")

        await setup.join_wifi(
            network.ssid, "wifi-password", network.security, ZoneInfo("Europe/London")
        )
        ```
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    @property
    def address(self) -> Optional[Tuple[str, int]]:
        """Returns the IP address and the port that the handler reaches the
        device at, on its setup network, or `None` over Bluetooth."""

    async def get_device_info(self) -> DeviceInfoBasicResult:
        """Returns *device info* as `DeviceInfoBasicResult`, which names the device's model.
        It is not guaranteed to contain all the properties returned from the Tapo API.
        If the deserialization fails, or if a property that you care about it's not present,
        try `SetupHandler.get_device_info_json`.

        Returns:
            DeviceInfoBasicResult: Basic device info of the device.
        """

    async def get_device_info_json(self) -> dict:
        """Returns *device info* as json.
        It contains all the properties returned from the Tapo API.

        Returns:
            dict: Device info as a dictionary.
        """

    async def scan_wifi(self) -> List[WifiNetwork]:
        """Returns the Wi-Fi networks that the device sees.

        Returns:
            List[WifiNetwork]: The Wi-Fi networks that the device sees.
        """

    async def join_wifi(
        self, ssid: str, password: str, security: WifiSecurity, timezone: ZoneInfo
    ) -> WifiJoinResult:
        """Puts the device on the Wi-Fi network `ssid`, with the `ApiClient`
        credentials as its login from then on.

        Over its setup network, the device leaves without answering, so this
        returns `WifiJoinResult.Sent` once the connection to the device drops
        or times out, which can take as long as the `ApiClient` `timeout_s`.
        Reconnect the host to its own network to reach the device again.

        Over Bluetooth, the device says whether it joined, so this waits up
        to 65 seconds for it and returns `WifiJoinResult.Connected`.

        Args:
            ssid (str): The name of the network, as in `WifiNetwork.ssid`.
            password (str): The password of the network, empty for an open network.
            security (WifiSecurity): The security of the network, as in `WifiNetwork.security`.
            timezone (ZoneInfo): The time zone that the device keeps its time in,
                for example for its schedules. The device takes the time itself
                from the host's clock.

        Returns:
            WifiJoinResult: How far the device has confirmed joining the network.

        Raises:
            Exception: If `ssid` is empty, if it holds U+FFFD, which
                `SetupHandler.scan_wifi` puts in place of a name that is not UTF-8,
                if `security` is `WifiSecurity.Unknown`, if `password` is empty for
                a network that is not open, if the device turns the request down,
                or, over Bluetooth, if the device does not join the network.
        """
