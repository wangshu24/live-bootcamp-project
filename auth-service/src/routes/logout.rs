use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie, CookieJar};

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        None => return (jar, Err(AuthAPIError::MissingToken)),
        Some(token) => token,
    };

    let token = cookie.value().to_owned();
    if validate_token(&token).await.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken));
    };

    let jar = jar.remove(cookie::Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::OK))
}
