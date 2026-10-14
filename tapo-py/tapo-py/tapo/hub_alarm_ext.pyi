from typing import List, Optional, Protocol

from tapo.requests import AlarmDuration, AlarmRingtone, AlarmVolume

class HubAlarmExt(Protocol):
    """Extension class for the hub alarm."""

    async def get_supported_ringtone_list(self) -> List[str]:
        """Returns a list of ringtones (alarm types) supported by the hub.
        This information is useful in debugging or when investigating new functionality to add.

        Returns:
            List[str]: List of the ringtones supported by the hub.
        """

    async def play_alarm(
        self,
        ringtone: AlarmRingtone,
        volume: AlarmVolume,
        duration: AlarmDuration,
        seconds: Optional[int] = None,
    ) -> None:
        """Start playing the hub alarm.

        Args:
            ringtone (AlarmRingtone): The ringtone of the alarm.
            volume (AlarmVolume): The volume of the alarm.
            duration (AlarmDuration): Controls how long the alarm plays for.
            seconds (Optional[int]): Play the alarm a number of seconds. Required if `duration` is `AlarmDuration.Seconds`.
        """

    async def stop_alarm(self) -> None:
        """Stop playing the hub alarm, if it's currently playing."""
