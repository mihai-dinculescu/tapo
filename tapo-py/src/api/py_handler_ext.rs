use std::sync::Arc;

use tapo::HandlerExt;
use tokio::sync::RwLock;

pub trait PyHandlerExt: Send + Sync + Sized {
    fn get_inner_handler(&self) -> Arc<RwLock<(impl HandlerExt + 'static)>>;
}
