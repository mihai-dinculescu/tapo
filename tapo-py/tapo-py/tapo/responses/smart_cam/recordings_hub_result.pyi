from datetime import date, datetime
from enum import Enum

from tapo.to_dict_ext import ToDictExt

class RecordingDateHubResult(ToDictExt):
    """A day on a camera hub's calendar that has recordings for a camera paired to it."""

    date: date
    """The day, on the hub's calendar."""

    start_time: datetime
    """When the day starts on the hub, in UTC."""

    end_time: datetime
    """The last second of the day on the hub, in UTC."""

class RecordingType(str, Enum):
    """The type of event that produced a recording. The names follow the
    Tapo app's playback event table. A type this library does not know yet
    is ``Other``; its raw wire value is still in ``to_dict()["video_type"]``."""

    Timing = "Timing"
    """Continuous (timing) recording."""
    Motion = "Motion"
    """Motion detection."""
    Tamper = "Tamper"
    """Camera tampering."""
    LineCrossing = "LineCrossing"
    """Line crossing detection."""
    AreaIntrusion = "AreaIntrusion"
    """Area intrusion detection."""
    Person = "Person"
    """Person detection."""
    BabyCry = "BabyCry"
    """Baby cry detection."""
    Vehicle = "Vehicle"
    """Vehicle detection."""
    Pet = "Pet"
    """Pet detection."""
    RingAlarm = "RingAlarm"
    """Ring alarm."""
    Bark = "Bark"
    """Bark detection."""
    Meow = "Meow"
    """Meow detection."""
    GlassBreaking = "GlassBreaking"
    """Glass breaking detection."""
    Smoke = "Smoke"
    """Smoke alarm detection."""
    PackageDelivered = "PackageDelivered"
    """Package delivered."""
    PackagePickedUp = "PackagePickedUp"
    """Package picked up."""
    MissedDoorbellRing = "MissedDoorbellRing"
    """Missed doorbell ring."""
    AnsweredDoorbellRing = "AnsweredDoorbellRing"
    """Answered doorbell ring."""
    AntiTheft = "AntiTheft"
    """Anti-theft alarm."""
    Face = "Face"
    """Face detection."""
    UnfamiliarFace = "UnfamiliarFace"
    """Unfamiliar face detection."""
    UnfamiliarPerson = "UnfamiliarPerson"
    """Unfamiliar person detection."""
    BabyLeave = "BabyLeave"
    """Baby leaving detection."""
    BabyCaregiver = "BabyCaregiver"
    """Baby caregiver detection."""
    BabyAsleep = "BabyAsleep"
    """Baby asleep detection."""
    BabyAwake = "BabyAwake"
    """Baby waking up detection."""
    FaceCover = "FaceCover"
    """Covered face detection."""
    SafeFenceOut = "SafeFenceOut"
    """Leaving the safety fence detection."""
    BabyMotion = "BabyMotion"
    """Baby motion detection."""
    PanoramicVideo = "PanoramicVideo"
    """Panoramic video."""
    Animal = "Animal"
    """Animal detection."""
    Other = "Other"
    """A recording type this library does not know yet."""

class RecordingHubResult(ToDictExt):
    """Recording stored on a camera hub for a camera paired to it."""

    start_time: datetime
    """Start of the recording."""

    end_time: datetime
    """End of the recording."""

    video_type: RecordingType
    """The type of event that produced the recording."""
