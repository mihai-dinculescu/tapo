from typing import List, Optional

from tapo.to_dict_ext import ToDictExt

class IrRemoteResult(ToDictExt):
    """Device info of the IR remotes paired with a Tapo H110 hub.

    IR remotes are virtual child devices (`SMART.TAPOREMOTE`) that are created by
    the Tapo app, either by picking a device from TP-Link's IR database or by
    learning the keys from a physical remote. They have no firmware or hardware
    of their own, which is why `fw_ver`, `hw_id` and `hw_ver` are always empty.

    Specific properties: `key_list`, `key_sum`, `customize_key_sum`,
    `downloaded_key_sum`, `remote_id`, `remote_type`.
    """

    avatar: str
    bind_count: int
    category: str
    device_id: str
    fw_ver: str
    hw_id: str
    hw_ver: str
    mac: str
    model: str
    """The kind of appliance the remote controls, as named by the Tapo app.
    Unlike the other hub child devices, this is not a Tapo model
    (e.g. "TV", "AV", "Light")."""
    nickname: str
    parent_device_id: str
    type: str

    last_onboarding_timestamp: int
    key_list: List[IrRemoteKey]
    """The keys stored on this remote."""
    key_sum: int
    """The total number of keys stored on this remote."""
    customize_key_sum: int
    """The number of keys that were learned from a physical remote."""
    downloaded_key_sum: int
    """The number of keys that were downloaded from TP-Link's IR database."""
    remote_id: int
    """The id of the appliance in TP-Link's IR database, or `0` for a generic one."""
    remote_type: int

class IrRemoteKey(ToDictExt):
    """A key stored on an IR remote paired with a Tapo H110 hub."""

    id: int
    """The id of the key in TP-Link's IR database, or `-1` for a key that was
    learned from a physical remote."""
    name: str
    """The name of the key. This is the value that
    `IrRemoteHandler.send_ir_cmd_by_id` expects."""
    display_name: str
    """The label shown in the Tapo app. It is meaningful for most downloaded keys
    (e.g. "POWER", "VOL+"), but it can also be an opaque string, in which case
    `IrRemoteKey.name` is the only usable identifier."""
    pwm: int
    """The carrier frequency of the key, in kHz."""
    icon: Optional[str]
    """The icon shown in the Tapo app. Only set for keys that were learned from
    a physical remote."""
    order: Optional[int]
    """The position of the key in the Tapo app. Only set for keys that were
    learned from a physical remote."""
    type: Optional[str]
