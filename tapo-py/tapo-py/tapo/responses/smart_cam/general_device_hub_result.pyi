from tapo.to_dict_ext import ToDictExt

class AiCameraSupport:
    """Bitmask of the AI detection types a camera paired to a camera hub
    runs itself: `1` person, `2` pet, `4` vehicle, `8` face.
    Bits above `8` are reserved for detection types not yet known."""

    person: bool
    """Whether the camera runs person detection."""

    pet: bool
    """Whether the camera runs pet detection."""

    vehicle: bool
    """Whether the camera runs vehicle detection."""

    face: bool
    """Whether the camera runs face detection."""

    raw: int
    """The raw bitmask value."""

class BackupWifi:
    """The backup Wi-Fi network of a camera paired to a camera hub.

    Check the variant with ``isinstance``, e.g.
    ``isinstance(camera.backup_wifi, BackupWifi.Ssid)``."""

    class Auto:
        """The backup network is chosen automatically."""

        def __init__(self) -> None: ...

    class Disabled:
        """No backup network is configured."""

        def __init__(self) -> None: ...

    class Ssid:
        """A specific configured SSID."""

        ssid: str

        def __init__(self, ssid: str) -> None: ...

class GeneralDeviceHubResult(ToDictExt):
    """General device (standalone Wi-Fi camera) paired to a camera hub."""

    ai_camera_support: AiCameraSupport
    """The AI detection types the camera runs itself."""

    alias: str

    backup_wifi: BackupWifi
    """The backup Wi-Fi network. In ``to_dict`` it is the raw value: ``"auto"``,
    ``""`` when disabled, or the SSID."""

    category: str
    device_id: str
    device_model: str
    device_type: str

    hub_storage_enabled: bool
    """Whether the hub stores this camera's footage."""

    mac: str
    network_mode: str
    parent_device_id: str

    plan_24h_record: bool
    """Whether 24h continuous recording is enabled for this camera."""

    wifi_backup_enabled: bool
