from typing import Optional

from tapo.responses.device_info_result.device_info_ext import DeviceInfoExt
from tapo.to_dict_ext import ToDictExt

class DeviceInfoBasicResult(DeviceInfoExt, ToDictExt):
    """Basic device info of a Tapo device."""

    nickname: Optional[str]
