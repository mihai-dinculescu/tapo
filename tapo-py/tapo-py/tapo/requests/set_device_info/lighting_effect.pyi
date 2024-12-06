from enum import Enum
from typing import List, Optional, Tuple

class LightingEffectType(str, Enum):
    Sequence = "Sequence"
    Random = "Random"
    Pulse = "Pulse"
    Static = "Static"

class LightingEffect:
    brightness: int
    is_custom: bool
    display_colors: List[Tuple[int, int, int]]
    """The colors that will be displayed in the Tapo app."""
    enabled: bool
    id: str
    name: str
    type: str
    backgrounds: Optional[List[Tuple[int, int, int]]]
    brightness_range: Optional[List[Tuple[int, int]]]
    direction: Optional[int]
    duration: Optional[int]
    expansion_strategy: Optional[int]
    fade_off: Optional[int]
    hue_range: Optional[List[Tuple[int, int]]]
    init_states: Optional[List[Tuple[int, int, int]]]
    random_seed: Optional[int]
    repeat_times: Optional[int]
    run_time: Optional[int]
    saturation_range: Optional[List[Tuple[int, int]]]
    segment_length: Optional[int]
    segments: Optional[List[int]]
    sequence: Optional[List[Tuple[int, int, int]]]
    spread: Optional[int]
    transition: Optional[int]
    transition_range: Optional[List[Tuple[int, int]]]
    transition_sequence: Optional[List[int]]

    def __init__(
        self,
        name: str,
        type: LightingEffectType,
        is_custom: bool,
        enabled: bool,
        brightness: int,
        display_colors: List[Tuple[int, int, int]],
    ) -> None: ...
    def with_brightness(self, brightness: int) -> LightingEffect: ...
    def with_is_custom(self, is_custom: bool) -> LightingEffect: ...
    def with_display_colors(self, display_colors: List[Tuple[int, int, int]]) -> LightingEffect: ...
    def with_enabled(self, enabled: bool) -> LightingEffect: ...
    def with_id(self, id: str) -> LightingEffect: ...
    def with_name(self, name: str) -> LightingEffect: ...
    def with_type(self, type: LightingEffectType) -> LightingEffect: ...
    def with_backgrounds(self, backgrounds: List[Tuple[int, int, int]]) -> LightingEffect: ...
    def with_brightness_range(self, brightness_range: Tuple[int, int]) -> LightingEffect: ...
    def with_direction(self, direction: int) -> LightingEffect: ...
    def with_duration(self, duration: int) -> LightingEffect: ...
    def with_expansion_strategy(self, expansion_strategy: int) -> LightingEffect: ...
    def with_fade_off(self, fade_off: int) -> LightingEffect: ...
    def with_hue_range(self, hue_range: Tuple[int, int]) -> LightingEffect: ...
    def with_init_states(self, init_states: List[Tuple[int, int, int]]) -> LightingEffect: ...
    def with_random_seed(self, random_seed: int) -> LightingEffect: ...
    def with_repeat_times(self, repeat_times: int) -> LightingEffect: ...
    def with_run_time(self, run_time: int) -> LightingEffect: ...
    def with_saturation_range(self, saturation_range: Tuple[int, int]) -> LightingEffect: ...
    def with_segment_length(self, segment_length: int) -> LightingEffect: ...
    def with_segments(self, segments: List[int]) -> LightingEffect: ...
    def with_sequence(self, sequence: List[Tuple[int, int, int]]) -> LightingEffect: ...
    def with_spread(self, spread: int) -> LightingEffect: ...
    def with_transition(self, transition: int) -> LightingEffect: ...
    def with_transition_range(self, transition_range: Tuple[int, int]) -> LightingEffect: ...
    def with_transition_sequence(self, transition_sequence: List[int]) -> LightingEffect: ...

class LightingEffectPreset(str, Enum):
    Aurora = "Aurora"
    BubblingCauldron = "BubblingCauldron"
    CandyCane = "CandyCane"
    Christmas = "Christmas"
    Flicker = "Flicker"
    GrandmasChristmasLights = "GrandmasChristmasLights"
    Hanukkah = "Hanukkah"
    HauntedMansion = "HauntedMansion"
    Icicle = "Icicle"
    Lightning = "Lightning"
    Ocean = "Ocean"
    Rainbow = "Rainbow"
    Raindrop = "Raindrop"
    Spring = "Spring"
    Sunrise = "Sunrise"
    Sunset = "Sunset"
    Valentines = "Valentines"
