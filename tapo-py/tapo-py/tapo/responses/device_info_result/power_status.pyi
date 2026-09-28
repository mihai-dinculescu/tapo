from typing import Final

class ChargingStatus:
    Finished: Final[ChargingStatus]
    Normal: Final[ChargingStatus]

class OvercurrentStatus:
    Lifted: Final[OvercurrentStatus]
    Normal: Final[OvercurrentStatus]

class OverheatStatus:
    CoolDown: Final[OverheatStatus]
    Normal: Final[OverheatStatus]
    Overheated: Final[OverheatStatus]

class PowerProtectionStatus:
    Normal: Final[PowerProtectionStatus]
    Overloaded: Final[PowerProtectionStatus]
