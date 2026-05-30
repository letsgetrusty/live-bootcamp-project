use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{app_state::AppState, domain::error::AuthAPIError, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}};


pub async fn logout(
    State(_state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

    let token = cookie.value().to_owned();

    validate_token(&token, _state.banned_token_store.clone()).await.map_err(|_| AuthAPIError::InvalidToken)?;

    let jar = jar.remove(JWT_COOKIE_NAME);

    _state.banned_token_store.store_token(&token).await.map_err(|e| {
        match e {
            crate::services::data_store::BannedTokenStoreError::TokenAlreadyBanned => AuthAPIError::InvalidToken,
            crate::services::data_store::BannedTokenStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
        }
    } )?;

    Ok((jar, StatusCode::OK))
}
