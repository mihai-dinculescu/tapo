from enum import Enum
from typing import List, Optional, Tuple

class SegmentEffectType(str, Enum):
    Circulating = "Circulating"
    Breathe = "Breathe"
    Chasing = "Chasing"
    Flicker = "Flicker"
    Bloom = "Bloom"
    Stacking = "Stacking"

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

class SegmentEffectPreset(str, Enum):
    Birthday = "Birthday"
    Blue = "Blue"
    Bonfire = "Bonfire"
    Candlelight = "Candlelight"
    Carnival = "Carnival"
    Cyan = "Cyan"
    Dancing = "Dancing"
    Dating = "Dating"
    Disco = "Disco"
    Dreamland = "Dreamland"
    ElectroDance = "ElectroDance"
    Energetic = "Energetic"
    Excited = "Excited"
    Fall = "Fall"
    Family = "Family"
    Fireworks = "Fireworks"
    FlowerField = "FlowerField"
    Forest = "Forest"
    Game = "Game"
    Green = "Green"
    Halloween = "Halloween"
    Happy = "Happy"
    Jazz = "Jazz"
    Lake = "Lake"
    LightGreen = "LightGreen"
    Lyric = "Lyric"
    Moonlight = "Moonlight"
    Morning = "Morning"
    Movie = "Movie"
    NewYear = "NewYear"
    Night = "Night"
    Orange = "Orange"
    Pink = "Pink"
    Purple = "Purple"
    Quiet = "Quiet"
    Red = "Red"
    Relaxed = "Relaxed"
    Rock = "Rock"
    Siren = "Siren"
    Sleep = "Sleep"
    Snow = "Snow"
    Star = "Star"
    Study = "Study"
    Summer = "Summer"
    Sunny = "Sunny"
    Sweet = "Sweet"
    Tense = "Tense"
    Thinking = "Thinking"
    Universe = "Universe"
    Volcano = "Volcano"
    Warm = "Warm"
    White = "White"
    Winter = "Winter"
    Work = "Work"
    Yellow = "Yellow"
