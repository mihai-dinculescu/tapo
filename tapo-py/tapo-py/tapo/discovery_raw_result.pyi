from __future__ import annotations
from typing import Any

from .to_dict_ext import ToDictExt

class DiscoveryRawResult(ToDictExt):
    @property
    def ip(self) -> str: ...
    @property
    def message(self) -> dict[str, Any]: ...

class MaybeDiscoveryRawResult:
    def get(self) -> DiscoveryRawResult: ...
