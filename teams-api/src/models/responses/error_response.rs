use serde::Deserialize;

/// Defines an HTTP API response.
#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    /// An Error object that contains information about the error.
    pub error: Error,
}

/// Object representing error information.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// Error code.
    pub code: String,
    /// Object representing the inner HTTP error.
    pub inner_http_error: InnerHttpError,
    /// A description of the error.
    pub message: String,
}

/// Object representing an inner HTTP error.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerHttpError {
    /// HTTP status code from the failed request.
    pub status_code: u16,
    /// Body from the failed request.
    pub body: serde_json::Value,
}
