use super::ApiClientExt;

/// Implemented by all device handlers.
pub trait HandlerExt: Send + Sync {
    /// Returns the client used by this handler.
    fn get_client(&self) -> &dyn ApiClientExt;
}
