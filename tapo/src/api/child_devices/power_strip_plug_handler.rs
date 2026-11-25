use crate::responses::PowerStripPlugResult;

tapo_child_handler! {
    /// Handler for the [P300](https://www.tp-link.com/en/search/?q=P300),
    /// [P306](https://www.tp-link.com/us/search/?q=P306) and
    /// [TP25](https://www.tapo.com/en/search/?q=TP25) child plugs.
    PowerStripPlugHandler(PowerStripPlugResult),
    on_off,
}
