use async_trait::async_trait;
use tokio::sync::RwLockReadGuard;

use super::ApiClientExt;

/// Implemented by all device handlers.
#[async_trait]
pub trait HandlerExt: Send + Sync {
    /// Returns the client used by this handler.
    async fn get_client(&self) -> RwLockReadGuard<'_, dyn ApiClientExt>;
}
