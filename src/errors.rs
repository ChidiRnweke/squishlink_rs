use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    DatabaseError(diesel::result::Error),
    UserInputError(String),
    NotFoundError,
    InfraError(String),
}

impl AppError {
    fn log_error(&self) {
        match self {
            AppError::DatabaseError(e) => log::error!("{e}"),
            AppError::InfraError(e) => log::error!("{e}"),
            _ => (),
        }
    }
}

const NOT_FOUND_ERR_MSG: & str =
    "The resource you're looking for can't be found. Maybe it was already deleted? Links only stay valid for 7 days.";
const DB_ERR_MSG: &str =
    "An unexpected error occurred. If this persists please reach out and let me know.";
const INPUT_ERR_MSG: & str  = "Something went wrong while trying to read your input. Is it a valid uri? (it must start with http:// or https://)";

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.log_error();
        let res = match self {
            AppError::NotFoundError => (StatusCode::NOT_FOUND, NOT_FOUND_ERR_MSG),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, DB_ERR_MSG),
            AppError::UserInputError(_) => (StatusCode::BAD_REQUEST, INPUT_ERR_MSG),
            AppError::InfraError(_) => (StatusCode::INTERNAL_SERVER_ERROR, DB_ERR_MSG),
        };
        res.into_response()
    }
}
