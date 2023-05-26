use serde::Serialize;

use crate::requests::TapoRequest;

#[derive(Debug, Serialize)]
pub(crate) struct MultipleRequestParams {
    requests: Vec<TapoRequest>,
}

impl MultipleRequestParams {
    pub fn new(requests: Vec<TapoRequest>) -> Self {
        Self { requests }
    }
}
