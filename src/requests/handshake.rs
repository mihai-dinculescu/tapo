use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct HandshakeParams {
    key: String,
}

impl HandshakeParams {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
