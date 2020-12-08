use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoogleErrorResponse {
    pub error: GoogleError,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoogleError {
    code: u16,
    errors: Vec<ErrorItem>,
    message: String,
}

impl std::fmt::Display for GoogleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for GoogleError {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorItem {
    domain: String,
    message: String,
    reason: String,
}
