from typing import Final

class AlarmVolume:
    """The volume of the alarm.
    For the H100, this is a fixed list of volume levels."""

    Default: Final[AlarmVolume]
    """Use the default volume for the hub."""

    Mute: Final[AlarmVolume]
    """Mute the audio output from the alarm.
    This causes the alarm to be shown as triggered in the Tapo App
    without an audible sound, and makes the `in_alarm` property
    in `DeviceInfoHubResult` return as `True`."""

    Low: Final[AlarmVolume]
    """Lowest volume."""

    Normal: Final[AlarmVolume]
    """Normal volume. This is the default."""

    High: Final[AlarmVolume]
    """Highest volume."""

class AlarmRingtone:
    """The ringtone of an H100 alarm."""

    Alarm1: Final[AlarmRingtone]
    """Alarm 1"""

    Alarm2: Final[AlarmRingtone]
    """Alarm 2"""

    Alarm3: Final[AlarmRingtone]
    """Alarm 3"""

    Alarm4: Final[AlarmRingtone]
    """Alarm 4"""

    Alarm5: Final[AlarmRingtone]
    """Alarm 5"""

    Connection1: Final[AlarmRingtone]
    """Connection 1"""

    Connection2: Final[AlarmRingtone]
    """Connection 2"""

    DoorbellRing1: Final[AlarmRingtone]
    """Doorbell Ring 1"""

    DoorbellRing2: Final[AlarmRingtone]
    """Doorbell Ring 2"""

    DoorbellRing3: Final[AlarmRingtone]
    """Doorbell Ring 3"""

    DoorbellRing4: Final[AlarmRingtone]
    """Doorbell Ring 4"""

    DoorbellRing5: Final[AlarmRingtone]
    """Doorbell Ring 5"""

    DoorbellRing6: Final[AlarmRingtone]
    """Doorbell Ring 6"""

    DoorbellRing7: Final[AlarmRingtone]
    """Doorbell Ring 7"""

    DoorbellRing8: Final[AlarmRingtone]
    """Doorbell Ring 8"""

    DoorbellRing9: Final[AlarmRingtone]
    """Doorbell Ring 9"""

    DoorbellRing10: Final[AlarmRingtone]
    """Doorbell Ring 10"""

    DrippingTap: Final[AlarmRingtone]
    """Dripping Tap"""

    PhoneRing: Final[AlarmRingtone]
    """Phone Ring"""

class AlarmDuration:
    """Controls how long the alarm plays for."""

    Continuous: Final[AlarmDuration]
    """Play the alarm continuously until stopped."""

    Once: Final[AlarmDuration]
    """Play the alarm once.
    This is useful for previewing the audio.

    Limitations:

    The `in_alarm` field of `DeviceInfoHubResult` will not remain `True` for the
    duration of the audio track. Each audio track has a different runtime.

    Has no observable affect when used in conjunction with `AlarmVolume.Mute`."""

    Seconds: Final[AlarmDuration]
    """Play the alarm a number of seconds."""
