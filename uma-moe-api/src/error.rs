use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("API error (status {status}): {error}")]
    Api {
        error: String,
        status: u16,
        details: Option<String>,
    },

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
