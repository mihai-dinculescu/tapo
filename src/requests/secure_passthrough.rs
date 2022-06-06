use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SecurePassthroughParams {
    request: String,
}

impl SecurePassthroughParams {
    pub fn new(request: &str) -> Self {
        Self {
            request: request.to_string(),
        }
    }
}
