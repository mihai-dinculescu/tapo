use std::fmt;

use serde::Serialize;

/// Implemented by all Tapo devices.
pub trait TapoDeviceExt: fmt::Debug + Default + Serialize {}
