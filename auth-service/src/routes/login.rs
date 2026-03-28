use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::{AuthAPIError, Email, Password};
use crate::AppState;

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let store = state.user_store.read().await;
    if store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    };

    let user = match store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}
