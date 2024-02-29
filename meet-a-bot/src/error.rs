use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    TeamsApi(#[from] teams_api::Error),
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
