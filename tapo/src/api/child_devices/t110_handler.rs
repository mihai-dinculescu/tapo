use crate::responses::{T110Event, T110Result};

tapo_child_handler! {
    /// Handler for the [T110](https://www.tapo.com/en/search/?q=T110) devices.
    T110Handler(T110Result),
    trigger_logs = T110Event,
}
