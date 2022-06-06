use serde::Deserialize;

/// Implemented by all Tapo Responses.
pub trait TapoResponseExt {}

impl TapoResponseExt for serde_json::Value {}

#[derive(Debug, Deserialize)]
pub struct TapoResponse<T: TapoResponseExt> {
    pub error_code: i32,
    pub result: Option<T>,
}

pub fn validate_result<T: TapoResponseExt>(response: &TapoResponse<T>) -> anyhow::Result<()> {
    if response.error_code != 0 {
        Err(anyhow::anyhow!(
            "expected error code 0, got {}",
            response.error_code
        ))
    } else {
        Ok(())
    }
}
