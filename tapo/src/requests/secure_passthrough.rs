use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct SecurePassthroughParams {
    request: String,
}

impl SecurePassthroughParams {
    pub fn new(request: &str) -> Self {
        Self {
            request: request.to_string(),
        }
    }
}
