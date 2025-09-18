from dataclasses import dataclass
from typing import Type, Union

from tapo import (
    ColorLightHandler,
    GenericDeviceHandler,
    HubHandler,
    LightHandler,
    PlugEnergyMonitoringHandler,
    PlugHandler,
    PowerStripEnergyMonitoringHandler,
    PowerStripHandler,
    RgbicLightStripHandler,
    RgbLightStripHandler,
)
from tapo.responses import (
    DeviceInfoColorLightResult,
    DeviceInfoGenericResult,
    DeviceInfoHubResult,
    DeviceInfoLightResult,
    DeviceInfoPlugEnergyMonitoringResult,
    DeviceInfoPlugResult,
    DeviceInfoPowerStripResult,
    DeviceInfoRgbicLightStripResult,
    DeviceInfoRgbLightStripResult,
)

class MaybeDiscoveryResult:
    """Potential result of the device discovery process. Using `get` will return the actual result or raise an exception."""

    def get(
        self,
    ) -> Union[
        GenericDevice,
        Light,
        ColorLight,
        RgbLightStrip,
        RgbicLightStrip,
        Plug,
        PlugEnergyMonitoring,
        PowerStrip,
        PowerStripEnergyMonitoring,
        Hub,
    ]:
        """Retrieves the actual discovery result or raises an exception."""

@dataclass
class GenericDevice:
    """A Generic Tapo device.

    If you believe this device is already supported, or would like to explore adding support for a currently
    unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
    to start the discussion.
    """

    device_info: DeviceInfoGenericResult
    """Device info of a Generic Tapo device.
    
    If you believe this device is already supported, or would like to explore adding support for a currently
    unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
    to start the discussion.
    """

    handler: GenericDeviceHandler
    """Handler for generic devices. It provides the functionality common to all Tapo [devices](https://www.tapo.com/en/).
    
    If you believe this device is already supported, or would like to explore adding support for a currently
    unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
    to start the discussion.
    """

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class Light:
    """Tapo L510, L520 and L610 devices."""

    device_info: DeviceInfoLightResult
    """Device info of Tapo L510, L520 and L610."""

    handler: LightHandler
    """Handler for the [L510](https://www.tapo.com/en/search/?q=L510),
    [L520](https://www.tapo.com/en/search/?q=L520) and
    [L610](https://www.tapo.com/en/search/?q=L610) devices.
    """

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class ColorLight:
    """Tapo L530, L535 and L630 devices."""

    device_info: DeviceInfoColorLightResult
    """Device info of Tapo L530, L535 and L630."""

    handler: ColorLightHandler
    """Handler for the [L530](https://www.tapo.com/en/search/?q=L530),
    [L535](https://www.tapo.com/en/search/?q=L535) and
    [L630](https://www.tapo.com/en/search/?q=L630) devices.
    """

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class RgbLightStrip:
    """Tapo L900 devices."""

    device_info: DeviceInfoRgbLightStripResult
    """Device info of Tapo L900."""

    handler: RgbLightStripHandler
    """Handler for the [L900](https://www.tapo.com/en/search/?q=L900) devices."""

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class RgbicLightStrip:
    """Tapo L920 and L930 devices."""

    device_info: DeviceInfoRgbicLightStripResult
    """Device info of Tapo L920 and L930."""

    handler: RgbicLightStripHandler
    """Handler for the [L920](https://www.tapo.com/en/search/?q=L920) and
    [L930](https://www.tapo.com/en/search/?q=L930) devices."""

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class Plug:
    """Tapo P100 and P105 devices."""

    device_info: DeviceInfoPlugResult
    """Device info of Tapo P100 and P105."""

    handler: PlugHandler
    """Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
    [P105](https://www.tapo.com/en/search/?q=P105) devices."""

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class PlugEnergyMonitoring:
    """Tapo P110, P110M and P115 devices."""

    device_info: DeviceInfoPlugEnergyMonitoringResult
    """Device info of Tapo P110, P110M and P115."""

    handler: PlugEnergyMonitoringHandler
    """Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
    [P110M](https://www.tapo.com/en/search/?q=P110M) and
    [P115](https://www.tapo.com/en/search/?q=P115) devices."""

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class PowerStrip:
    """Tapo P300 and P306 devices."""

    device_info: DeviceInfoPowerStripResult
    """Device info of Tapo P300 and P306."""

    handler: PowerStripHandler
    """Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
    [P306](https://www.tp-link.com/us/search/?q=P306) devices.
    """

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class PowerStripEnergyMonitoring:
    """Tapo P304M and P316M devices."""

    device_info: DeviceInfoPowerStripResult
    """Device info of Tapo P304M and P316M."""

    handler: PowerStripEnergyMonitoringHandler
    """Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
    [P316M](https://www.tp-link.com/us/search/?q=P316M) devices.
    """

    __match_args__ = (
        "device_info",
        "handler",
    )

@dataclass
class Hub:
    """Tapo H100 devices."""

    device_info: DeviceInfoHubResult
    """Device info of Tapo H100."""

    handler: HubHandler
    """Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices."""

    __match_args__ = (
        "device_info",
        "handler",
    )

class DiscoveryResult:
    """Result of the device discovery process."""

    GenericDevice: Type[GenericDevice] = GenericDevice
    Light: Type[Light] = Light
    ColorLight: Type[ColorLight] = ColorLight
    RgbLightStrip: Type[RgbLightStrip] = RgbLightStrip
    RgbicLightStrip: Type[RgbicLightStrip] = RgbicLightStrip
    Plug: Type[Plug] = Plug
    PlugEnergyMonitoring: Type[PlugEnergyMonitoring] = PlugEnergyMonitoring
    PowerStrip: Type[PowerStrip] = PowerStrip
    PowerStripEnergyMonitoring: Type[PowerStripEnergyMonitoring] = PowerStripEnergyMonitoring
    Hub: Type[Hub] = Hub
