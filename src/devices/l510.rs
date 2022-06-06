use std::fmt;

use serde::Serialize;

use crate::devices::TapoDeviceExt;

/// [Tapo L510](https://www.tapo.com/uk/search/?q=L510) devices.
#[derive(fmt::Debug, Default, Serialize)]
pub struct L510;
impl TapoDeviceExt for L510 {}
