use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User},
};

pub async fn signup(
    // TODO: Use Axum's state extractor to pass in AppState
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Create a new `User` instance using data in the `request`

    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    // TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

//...

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(serde::Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
