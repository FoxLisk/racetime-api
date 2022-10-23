use http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RacetimeError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Endpoint parsing error: {0}")]
    ParseError(#[from] url::ParseError),
    #[error("Endpoint not found")]
    /// racetime.gg appears to have their API behind their normal webserver, so
    /// 404s return the _generic, user-facing_ 404 page
    NotFound,

    #[error("Bad status code (not 404) {0}")]
    /// any non-404 error
    UnexpectedStatus(StatusCode),

    #[error("HttpError: {0}")]
    HttpError(#[from] http::Error),

    #[error("Error deserializing body: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("Error serializing parameters: {0}")]
    ParameterSerializationError(#[from] serde_urlencoded::ser::Error),
}
