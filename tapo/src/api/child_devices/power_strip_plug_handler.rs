use crate::responses::PowerStripPlugResult;

tapo_child_handler! {
    /// Handler for the [P300](https://www.tp-link.com/en/search/?q=P300) and
    /// [P306](https://www.tp-link.com/us/search/?q=P306) child plugs.
    PowerStripPlugHandler(PowerStripPlugResult),
    on_off,
}
