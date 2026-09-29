use crate::responses::{S200Log, S200Result};

tapo_child_handler! {
    /// Handler for the [S200B](https://www.tapo.com/en/search/?q=S200B) and
    /// [S200D](https://www.tapo.com/en/search/?q=S200D) devices.
    S200Handler(S200Result),
    trigger_logs = S200Log,
}
