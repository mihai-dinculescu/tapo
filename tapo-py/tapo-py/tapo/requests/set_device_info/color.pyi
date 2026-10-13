from typing import Final, Tuple

class Color:
    """List of preset colors as defined in the Google Home app."""

    CoolWhite: Final[Color]
    """White light at 4000 K."""
    Daylight: Final[Color]
    """White light at 5000 K."""
    Ivory: Final[Color]
    """White light at 6000 K."""
    WarmWhite: Final[Color]
    """White light at 3000 K."""
    Incandescent: Final[Color]
    """White light at 2700 K."""
    Candlelight: Final[Color]
    """White light at 2500 K."""
    Snow: Final[Color]
    """White light at 6500 K."""
    GhostWhite: Final[Color]
    """White light at 6500 K."""
    AliceBlue: Final[Color]
    """Hue 208°, saturation 5%."""
    LightGoldenrod: Final[Color]
    """Hue 54°, saturation 28%."""
    LemonChiffon: Final[Color]
    """Hue 54°, saturation 19%."""
    AntiqueWhite: Final[Color]
    """White light at 5500 K."""
    Gold: Final[Color]
    """Hue 50°, saturation 100%."""
    Peru: Final[Color]
    """Hue 29°, saturation 69%."""
    Chocolate: Final[Color]
    """Hue 30°, saturation 100%."""
    SandyBrown: Final[Color]
    """Hue 27°, saturation 60%."""
    Coral: Final[Color]
    """Hue 16°, saturation 68%."""
    Pumpkin: Final[Color]
    """Hue 24°, saturation 90%."""
    Tomato: Final[Color]
    """Hue 9°, saturation 72%."""
    Vermilion: Final[Color]
    """Hue 4°, saturation 77%."""
    OrangeRed: Final[Color]
    """Hue 16°, saturation 100%."""
    Pink: Final[Color]
    """Hue 349°, saturation 24%."""
    Crimson: Final[Color]
    """Hue 348°, saturation 90%."""
    DarkRed: Final[Color]
    """Hue 0°, saturation 100%."""
    HotPink: Final[Color]
    """Hue 330°, saturation 58%."""
    Smitten: Final[Color]
    """Hue 329°, saturation 67%."""
    MediumPurple: Final[Color]
    """Hue 259°, saturation 48%."""
    BlueViolet: Final[Color]
    """Hue 271°, saturation 80%."""
    Indigo: Final[Color]
    """Hue 274°, saturation 100%."""
    LightSkyBlue: Final[Color]
    """Hue 202°, saturation 46%."""
    CornflowerBlue: Final[Color]
    """Hue 218°, saturation 57%."""
    Ultramarine: Final[Color]
    """Hue 254°, saturation 100%."""
    DeepSkyBlue: Final[Color]
    """Hue 195°, saturation 100%."""
    Azure: Final[Color]
    """Hue 210°, saturation 100%."""
    NavyBlue: Final[Color]
    """Hue 240°, saturation 100%."""
    LightTurquoise: Final[Color]
    """Hue 180°, saturation 26%."""
    Aquamarine: Final[Color]
    """Hue 159°, saturation 50%."""
    Turquoise: Final[Color]
    """Hue 174°, saturation 71%."""
    LightGreen: Final[Color]
    """Hue 120°, saturation 39%."""
    Lime: Final[Color]
    """Hue 75°, saturation 100%."""
    ForestGreen: Final[Color]
    """Hue 120°, saturation 75%."""

    def get_color_config(self) -> Tuple[int, int, int]:
        """Get the `hue`, `saturation`, and `color_temperature` of the color."""
