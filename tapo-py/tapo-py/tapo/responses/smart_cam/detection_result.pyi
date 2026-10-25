from datetime import datetime
from typing import List

from tapo.responses.smart_cam.recordings_hub_result import RecordingType
from tapo.to_dict_ext import ToDictExt

class DetectionResult(ToDictExt):
    """A detection stored on a camera's SD card."""

    start_time: datetime
    """Start of the detection."""

    end_time: datetime
    """End of the detection."""

    trigger: RecordingType
    """The event the camera files the detection under. When a person and
    motion are both detected, this is ``RecordingType.Person``."""

    events: List[RecordingType]
    """Every event detected in the span, e.g. ``[RecordingType.Motion, RecordingType.Person]``."""
