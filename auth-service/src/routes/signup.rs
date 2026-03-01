use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(serde::Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

pub async fn signup(Json(_body): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
