use crate::responses::{T100Event, T100Result};

tapo_child_handler! {
    /// Handler for the [T100](https://www.tapo.com/en/search/?q=T100) devices.
    T100Handler(T100Result),
    trigger_logs = T100Event,
}
