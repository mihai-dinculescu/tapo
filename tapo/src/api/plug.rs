/// Power strip plug.
pub enum Plug {
    ///  By Device ID.
    ByDeviceId(String),
    /// By Nickname.
    ByNickname(String),
    /// By Position.
    ByPosition(u8),
}
