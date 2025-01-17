//! Errors originating from API calls, parsing responses, and reading-or-writing to the file system.
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum OpenAIError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// OpenAI returns error object with details of API call failure
    #[error("{}: {}", .0.r#type, .0.message)]
    ApiError(ApiError),
    /// Error when a response cannot be deserialized into a Rust type
    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(serde_json::Error),
    /// Error on the client side when saving image to file system
    #[error("failed to save image: {0}")]
    ImageSaveError(String),
    /// Error on the client side when reading image from file system
    #[error("failed to read image: {0}")]
    ImageReadError(String),
}

/// OpenAI API returns error object on failure
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub r#type: String,
    pub param: Option<serde_json::Value>,
    pub code: Option<serde_json::Value>,
}

/// Wrapper to deserialize the error object nested in "error" JSON key
#[derive(Deserialize)]
pub(crate) struct WrappedError {
    pub(crate) error: ApiError,
}
