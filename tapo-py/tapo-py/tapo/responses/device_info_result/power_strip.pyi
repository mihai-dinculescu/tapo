from tapo.responses.device_info_result.device_info_ext import DeviceInfoSmartExt
from tapo.to_dict_ext import ToDictExt

class DeviceInfoPowerStripResult(DeviceInfoSmartExt, ToDictExt):
    """Device info of Tapo P210M, P300, P304M, P306, P316M and TP25."""

    nickname: str
    """The name of the power strip itself, which the Tapo app doesn't show.
    The app names the child plugs instead. Empty when the strip has no name."""
