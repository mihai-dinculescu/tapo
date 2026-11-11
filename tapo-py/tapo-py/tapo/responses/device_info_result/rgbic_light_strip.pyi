from typing import List, Optional

from tapo.responses.device_info_result.default_state import DefaultStateType
from tapo.responses.device_info_result.device_info_ext import DeviceInfoSmartExt
from tapo.requests.set_device_info.lighting_effect import LightingEffect
from tapo.to_dict_ext import ToDictExt

class DeviceInfoRgbicLightStripResult(DeviceInfoSmartExt, ToDictExt):
    """Device info of Tapo L920 and L930."""

    brightness: int
    color_temp_range: List[int]
    color_temp: int
    default_states: DefaultRgbicLightStripState
    """The default state of a device to be used when internet connectivity is lost after a power cut."""
    device_on: bool
    hue: Optional[int]
    lighting_effect: Optional[LightingEffectSummary]
    """The lighting effect that the device ran last, which is running now when
    its `enabled` is `True`. It is `None` when the device has not run one yet."""
    nickname: str
    overheated: bool
    saturation: Optional[int]

class LightingEffectSummary(ToDictExt):
    """The lighting effect that an L920 or L930 ran last, as its device info reports it."""

    brightness: int
    display_colors: List[List[int]]
    """The colors that are displayed in the Tapo app."""
    enabled: bool
    """Whether the lighting effect is running. It is `False` while the device
    is off, and once a color or another kind of effect has replaced it."""
    id: str
    is_custom: bool
    """Whether the lighting effect was created in the Tapo app, as opposed to
    being one of its presets."""
    name: str

class DefaultRgbicLightStripState(ToDictExt):
    """RGB IC Light Strip Default State."""

    type: DefaultStateType
    state: RgbicLightStripState

class RgbicLightStripState(ToDictExt):
    """RGB IC Light Strip State."""

    brightness: Optional[int]
    hue: Optional[int]
    saturation: Optional[int]
    color_temp: Optional[int]
    lighting_effect: Optional[LightingEffect]
