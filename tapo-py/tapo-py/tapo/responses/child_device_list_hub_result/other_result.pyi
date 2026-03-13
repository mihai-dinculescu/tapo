from tapo.to_dict_ext import ToDictExt

class OtherResult(ToDictExt):
    """Catch-all for unsupported devices. Open a GitHub issue to request support."""

    device_id: str
    model: str
    nickname: str
