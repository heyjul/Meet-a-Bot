use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("An error occured when trying to contact an external service.")]
    Service(serde_json::Value),

    #[error("The command `{0}` is not a valid command. Use the `help` command to know which ones are available.")]
    UnknownCommand(String),

    #[error("The value `{0}` is missing.")]
    MissingValue(&'static str),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self)).into_response()
    }
}
