use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie, CookieJar};

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

    // Validate token
    let token = cookie.value().to_owned();
    validate_token(&token, state.banned_token_store.clone())
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    // Add token to banned list
    state
        .banned_token_store
        .write()
        .await
        .add_token(token)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    // Remove jwt cookie
    let jar = jar.remove(cookie::Cookie::from(JWT_COOKIE_NAME));

    Ok((jar, StatusCode::OK))
}
