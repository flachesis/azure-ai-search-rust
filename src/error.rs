use reqwest::StatusCode;
use thiserror::Error;

/// Main error type for Azure Search operations
#[derive(Debug, Error)]
pub enum Error {
    /// Failed HTTP request to Azure Search
    #[error("Request failed with status {status}: {body}")]
    RequestFailed { status: StatusCode, body: String },

    /// Failed to parse URL
    #[error("Invalid URL: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Failed to serialize/deserialize JSON
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Reqwest HTTP client error
    #[error("HTTP client error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Azure Search specific error
    #[error("Azure Search error: {0}")]
    SearchError(String),
}

/// Result type alias for Azure Search operations
pub type Result<T> = std::result::Result<T, Error>;
