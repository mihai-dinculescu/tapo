from enum import Enum

class OvercurrentStatus(str, Enum):
    Lifted = "lifted"
    Normal = "normal"

class OverheatStatus(str, Enum):
    CoolDown = "cool_down"
    Normal = "normal"
    Overheated = "overheated"

class PowerProtectionStatus(str, Enum):
    Normal = "normal"
    Overloaded = "overloaded"
