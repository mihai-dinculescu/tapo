from typing import Final, List, Optional, Tuple

class SegmentEffectType:
    Circulating: Final[SegmentEffectType]
    Breathe: Final[SegmentEffectType]
    Chasing: Final[SegmentEffectType]
    Flicker: Final[SegmentEffectType]
    Bloom: Final[SegmentEffectType]
    Stacking: Final[SegmentEffectType]

class SegmentEffect:
    brightness: int
    is_custom: bool
    device_type: Optional[str]
    display_colors: List[Tuple[int, int, int, int]]
    enabled: bool
    id: str
    name: str
    segments: Optional[List[int]]
    states: Optional[List[Tuple[int, int, int, int]]]
    type: Optional[str]

    def __init__(
        self,
        name: str,
        type: SegmentEffectType,
        is_custom: bool,
        enabled: bool,
        brightness: int,
        display_colors: List[Tuple[int, int, int, int]],
    ) -> None: ...
    def with_brightness(self, brightness: int) -> SegmentEffect: ...
    def with_is_custom(self, is_custom: bool) -> SegmentEffect: ...
    def with_display_colors(
        self, display_colors: List[Tuple[int, int, int, int]]
    ) -> SegmentEffect: ...
    def with_enabled(self, enabled: bool) -> SegmentEffect: ...
    def with_id(self, id: str) -> SegmentEffect: ...
    def with_name(self, name: str) -> SegmentEffect: ...
    def with_type(self, type: SegmentEffectType) -> SegmentEffect: ...
    def with_device_type(self, device_type: str) -> SegmentEffect: ...
    def with_segments(self, segments: List[int]) -> SegmentEffect: ...
    def with_states(self, states: List[Tuple[int, int, int, int]]) -> SegmentEffect: ...

class SegmentEffectPreset:
    Birthday: Final[SegmentEffectPreset]
    Blue: Final[SegmentEffectPreset]
    Bonfire: Final[SegmentEffectPreset]
    Candlelight: Final[SegmentEffectPreset]
    Carnival: Final[SegmentEffectPreset]
    Cyan: Final[SegmentEffectPreset]
    Dancing: Final[SegmentEffectPreset]
    Dating: Final[SegmentEffectPreset]
    Disco: Final[SegmentEffectPreset]
    Dreamland: Final[SegmentEffectPreset]
    ElectroDance: Final[SegmentEffectPreset]
    Energetic: Final[SegmentEffectPreset]
    Excited: Final[SegmentEffectPreset]
    Fall: Final[SegmentEffectPreset]
    Family: Final[SegmentEffectPreset]
    Fireworks: Final[SegmentEffectPreset]
    FlowerField: Final[SegmentEffectPreset]
    Forest: Final[SegmentEffectPreset]
    Game: Final[SegmentEffectPreset]
    Green: Final[SegmentEffectPreset]
    Halloween: Final[SegmentEffectPreset]
    Happy: Final[SegmentEffectPreset]
    Jazz: Final[SegmentEffectPreset]
    Lake: Final[SegmentEffectPreset]
    LightGreen: Final[SegmentEffectPreset]
    Lyric: Final[SegmentEffectPreset]
    Moonlight: Final[SegmentEffectPreset]
    Morning: Final[SegmentEffectPreset]
    Movie: Final[SegmentEffectPreset]
    NewYear: Final[SegmentEffectPreset]
    Night: Final[SegmentEffectPreset]
    Orange: Final[SegmentEffectPreset]
    Pink: Final[SegmentEffectPreset]
    Purple: Final[SegmentEffectPreset]
    Quiet: Final[SegmentEffectPreset]
    Red: Final[SegmentEffectPreset]
    Relaxed: Final[SegmentEffectPreset]
    Rock: Final[SegmentEffectPreset]
    Siren: Final[SegmentEffectPreset]
    Sleep: Final[SegmentEffectPreset]
    Snow: Final[SegmentEffectPreset]
    Star: Final[SegmentEffectPreset]
    Study: Final[SegmentEffectPreset]
    Summer: Final[SegmentEffectPreset]
    Sunny: Final[SegmentEffectPreset]
    Sweet: Final[SegmentEffectPreset]
    Tense: Final[SegmentEffectPreset]
    Thinking: Final[SegmentEffectPreset]
    Universe: Final[SegmentEffectPreset]
    Volcano: Final[SegmentEffectPreset]
    Warm: Final[SegmentEffectPreset]
    White: Final[SegmentEffectPreset]
    Winter: Final[SegmentEffectPreset]
    Work: Final[SegmentEffectPreset]
    Yellow: Final[SegmentEffectPreset]
