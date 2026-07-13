use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use secrecy::SecretString;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[tracing::instrument(name = "Verify 2FA", skip_all)]
pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id.clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let two_fa_code =
        TwoFACode::parse(request.two_fa_code).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    let code_tuple = two_fa_code_store
        .get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if !code_tuple.0.eq(&login_attempt_id) || !code_tuple.1.eq(&two_fa_code) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    two_fa_code_store
        .remove_code(&email)
        .await
        .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;

    let cookie =
        generate_auth_cookie(&email).map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;

    let updated_jar = jar.add(cookie);

    Ok((updated_jar, ()))
}

#[derive(Debug, Deserialize)]
pub struct Verify2FARequest {
    pub email: SecretString,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: SecretString,
    #[serde(rename = "2FACode")]
    pub two_fa_code: SecretString,
}
