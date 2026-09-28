from datetime import date, datetime
from typing import Final

from tapo.to_dict_ext import ToDictExt

class RecordingDateHubResult(ToDictExt):
    """A day on a camera hub's calendar that has recordings for a camera paired to it."""

    date: date
    """The day, on the hub's calendar."""

    start_time: datetime
    """When the day starts on the hub, in UTC."""

    end_time: datetime
    """The last second of the day on the hub, in UTC."""

class RecordingType:
    """The type of event that produced a recording. The names follow the
    Tapo app's playback event table. A type this library does not know yet
    is ``Other``; its raw wire value is still in ``to_dict()["video_type"]``."""

    Timing: Final[RecordingType]
    """Continuous (timing) recording."""
    Motion: Final[RecordingType]
    """Motion detection."""
    Tamper: Final[RecordingType]
    """Camera tampering."""
    LineCrossing: Final[RecordingType]
    """Line crossing detection."""
    AreaIntrusion: Final[RecordingType]
    """Area intrusion detection."""
    Person: Final[RecordingType]
    """Person detection."""
    BabyCry: Final[RecordingType]
    """Baby cry detection."""
    Vehicle: Final[RecordingType]
    """Vehicle detection."""
    Pet: Final[RecordingType]
    """Pet detection."""
    RingAlarm: Final[RecordingType]
    """Ring alarm."""
    Bark: Final[RecordingType]
    """Bark detection."""
    Meow: Final[RecordingType]
    """Meow detection."""
    GlassBreaking: Final[RecordingType]
    """Glass breaking detection."""
    Smoke: Final[RecordingType]
    """Smoke alarm detection."""
    PackageDelivered: Final[RecordingType]
    """Package delivered."""
    PackagePickedUp: Final[RecordingType]
    """Package picked up."""
    MissedDoorbellRing: Final[RecordingType]
    """Missed doorbell ring."""
    AnsweredDoorbellRing: Final[RecordingType]
    """Answered doorbell ring."""
    AntiTheft: Final[RecordingType]
    """Anti-theft alarm."""
    Face: Final[RecordingType]
    """Face detection."""
    UnfamiliarFace: Final[RecordingType]
    """Unfamiliar face detection."""
    UnfamiliarPerson: Final[RecordingType]
    """Unfamiliar person detection."""
    BabyLeave: Final[RecordingType]
    """Baby leaving detection."""
    BabyCaregiver: Final[RecordingType]
    """Baby caregiver detection."""
    BabyAsleep: Final[RecordingType]
    """Baby asleep detection."""
    BabyAwake: Final[RecordingType]
    """Baby waking up detection."""
    FaceCover: Final[RecordingType]
    """Covered face detection."""
    SafeFenceOut: Final[RecordingType]
    """Leaving the safety fence detection."""
    BabyMotion: Final[RecordingType]
    """Baby motion detection."""
    PanoramicVideo: Final[RecordingType]
    """Panoramic video."""
    Animal: Final[RecordingType]
    """Animal detection."""
    Other: Final[RecordingType]
    """A recording type this library does not know yet."""

class RecordingHubResult(ToDictExt):
    """Recording stored on a camera hub for a camera paired to it."""

    start_time: datetime
    """Start of the recording."""

    end_time: datetime
    """End of the recording."""

    video_type: RecordingType
    """The type of event that produced the recording."""
