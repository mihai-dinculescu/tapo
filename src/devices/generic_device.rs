use std::fmt;

use serde::Serialize;

use crate::devices::TapoDeviceExt;

/// Basic functionality of all [Tapo devices](https://www.tapo.com/uk/).
#[derive(fmt::Debug, Default, Serialize)]
pub struct GenericDevice;
impl TapoDeviceExt for GenericDevice {}
