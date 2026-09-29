use crate::responses::{T300Event, T300Result};

tapo_child_handler! {
    /// Handler for the [T300](https://www.tapo.com/en/search/?q=T300) devices.
    T300Handler(T300Result),
    trigger_logs = T300Event,
}
