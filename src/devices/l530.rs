use std::fmt;

use serde::Serialize;

use crate::devices::TapoDeviceExt;

/// [Tapo L530](https://www.tapo.com/uk/search/?q=L530) devices.
#[derive(fmt::Debug, Default, Serialize)]
pub struct L530;
impl TapoDeviceExt for L530 {}
