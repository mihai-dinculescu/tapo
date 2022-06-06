use std::fmt;

use serde::Serialize;

use crate::devices::TapoDeviceExt;

/// [Tapo P110](https://www.tapo.com/uk/search/?q=P110) devices.
#[derive(fmt::Debug, Default, Serialize)]
pub struct P110;
impl TapoDeviceExt for P110 {}
