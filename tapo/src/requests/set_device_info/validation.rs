use std::fmt::Display;
use std::ops::RangeInclusive;

use crate::error::Error;

/// The brightness levels `set_brightness` and the `brightness` setter of
/// [`crate::requests::ColorLightSetDeviceInfoParams`] accept.
pub const BRIGHTNESS_RANGE: RangeInclusive<u8> = 1..=100;
/// The hues, in degrees, `set_hue_saturation` and the `hue_saturation` setter of
/// [`crate::requests::ColorLightSetDeviceInfoParams`] accept.
pub const HUE_RANGE: RangeInclusive<u16> = 0..=360;
/// The saturation levels `set_hue_saturation` and the `hue_saturation` setter of
/// [`crate::requests::ColorLightSetDeviceInfoParams`] accept.
pub const SATURATION_RANGE: RangeInclusive<u8> = 1..=100;
/// The color temperatures, in kelvin, `set_color_temperature` and the `color_temperature`
/// setter of [`crate::requests::ColorLightSetDeviceInfoParams`] accept.
pub const COLOR_TEMPERATURE_RANGE: RangeInclusive<u16> = 2500..=6500;

/// Returns a validation error for `field` if `value` is outside `range`.
pub(crate) fn check_range<T: PartialOrd + Display>(
    field: &str,
    range: &RangeInclusive<T>,
    value: T,
) -> Result<(), Error> {
    if range.contains(&value) {
        return Ok(());
    }

    Err(Error::Validation {
        field: field.to_string(),
        message: format!("Must be between {} and {}", range.start(), range.end()),
    })
}
