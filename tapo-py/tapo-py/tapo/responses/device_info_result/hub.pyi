from tapo.responses.device_info_result.device_info_ext import DeviceInfoExt
from tapo.to_dict_ext import ToDictExt

class DeviceInfoHubResult(DeviceInfoExt, ToDictExt):
    """Device info of Tapo H100."""

    in_alarm: bool
    in_alarm_source: str
    nickname: str
    overheated: bool
