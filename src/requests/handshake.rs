use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HandshakeParams {
    key: String,
}

impl HandshakeParams {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}
