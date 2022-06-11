use crate::devices::TapoDeviceExt;

/// Basic functionality of all [Tapo devices](https://www.tapo.com/uk/).
#[derive(Debug)]
pub struct GenericDevice;
impl TapoDeviceExt for GenericDevice {}
