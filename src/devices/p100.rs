use std::fmt;

use serde::Serialize;

use crate::devices::TapoDeviceExt;

/// [Tapo P100](https://www.tapo.com/uk/search/?q=P100) devices.
#[derive(fmt::Debug, Default, Serialize)]
pub struct P100;
impl TapoDeviceExt for P100 {}
