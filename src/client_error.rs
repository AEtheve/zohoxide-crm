use crate::response::ApiErrorResponse;
use thiserror::Error;

/// Various errors returned by the API.
#[derive(Error, Debug)]
pub enum ClientError {
    /// General error message that encompasses almost any non-token related error message.
    #[error("{0}")]
    General(String),

    /// Error returned when a response from the API does not deserialize into the user's
    /// custom data type. The raw response will be returned with this error.
    #[error("{0}")]
    UnexpectedResponseType(String),

    /// Error return when a response from the API is empty
    #[error("Empty response")]
    EmptyResponse,

    /// Error returned from most API requests.
    #[error("{0}")]
    ApiError(ApiErrorResponse),
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::General(err.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for ClientError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        ClientError::General(err.to_string())
    }
}

impl From<&str> for ClientError {
    fn from(err: &str) -> ClientError {
        ClientError::General(String::from(err))
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> ClientError {
        ClientError::General(err.to_string())
    }
}
