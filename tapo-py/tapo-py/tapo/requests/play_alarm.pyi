from enum import Enum

class AlarmVolume(str, Enum):
    """The volume of the alarm.
    For the H100, this is a fixed list of volume levels."""

    Default = "Default"
    """Use the default volume for the hub."""

    Mute = "Mute"
    """Mute the audio output from the alarm.
    This causes the alarm to be shown as triggered in the Tapo App
    without an audible sound, and makes the `in_alarm` property
    in `DeviceInfoHubResult` return as `True`."""

    Low = "Low"
    """Lowest volume."""

    Normal = "Normal"
    """Normal volume. This is the default."""

    High = "High"
    """Highest volume."""

class AlarmRingtone(str, Enum):
    """The ringtone of a H100 alarm."""

    Alarm1 = "Alarm1"
    """Alarm 1"""

    Alarm2 = "Alarm2"
    """Alarm 2"""

    Alarm3 = "Alarm3"
    """Alarm 3"""

    Alarm4 = "Alarm4"
    """Alarm 4"""

    Alarm5 = "Alarm5"
    """Alarm 5"""

    Connection1 = "Connection1"
    """Connection 1"""

    Connection2 = "Connection2"
    """Connection 2"""

    DoorbellRing1 = "DoorbellRing1"
    """Doorbell Ring 1"""

    DoorbellRing2 = "DoorbellRing2"
    """Doorbell Ring 2"""

    DoorbellRing3 = "DoorbellRing3"
    """Doorbell Ring 3"""

    DoorbellRing4 = "DoorbellRing4"
    """Doorbell Ring 4"""

    DoorbellRing5 = "DoorbellRing5"
    """Doorbell Ring 5"""

    DoorbellRing6 = "DoorbellRing6"
    """Doorbell Ring 6"""

    DoorbellRing7 = "DoorbellRing7"
    """Doorbell Ring 7"""

    DoorbellRing8 = "DoorbellRing8"
    """Doorbell Ring 8"""

    DoorbellRing9 = "DoorbellRing9"
    """Doorbell Ring 9"""

    DoorbellRing10 = "DoorbellRing10"
    """Doorbell Ring 10"""

    DrippingTap = "DrippingTap"
    """Dripping Tap"""

    PhoneRing = "PhoneRing"
    """Phone Ring"""

class AlarmDuration(str, Enum):
    """Controls how long the alarm plays for."""

    Continuous = "Continuous"
    """Play the alarm continuously until stopped."""

    Once = "Once"
    """Play the alarm once.
    This is useful for previewing the audio.

    Limitations:

    The `in_alarm` field of `DeviceInfoHubResult` will not remain `True` for the
    duration of the audio track. Each audio track has a different runtime.

    Has no observable affect when used in conjunction with `AlarmVolume.Mute`."""

    Seconds = "Seconds"
    """Play the alarm a number of seconds."""
