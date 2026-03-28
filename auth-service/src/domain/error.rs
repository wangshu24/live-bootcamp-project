use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        match self {
            AuthAPIError::UserAlreadyExists => StatusCode::CONFLICT.into_response(),
            AuthAPIError::InvalidCredentials => StatusCode::UNAUTHORIZED.into_response(),
            AuthAPIError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
