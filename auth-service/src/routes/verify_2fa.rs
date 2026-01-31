use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{email::Email, error::AuthAPIError}, services::data_store::{LoginAttemptId, TwoFACode}, utils::auth::generate_auth_cookie};


pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<(CookieJar, (StatusCode, Json<Verify2FAResponse>)), AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let login_attempt_id =
        LoginAttemptId::parse(request.login_attempt_id).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let two_fa_code =
        TwoFACode::parse(request.two_fa_code).map_err(|_| AuthAPIError::InvalidCredentials)?;

    // avoid moving out of state unless that's what you want
    let two_fa_code_store = state.two_fa_code_store.clone();

    let code_tuple = two_fa_code_store
        .lock().await
        .get_code(&email).await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if code_tuple.0 != login_attempt_id || code_tuple.1 != two_fa_code {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let auth_cookie = generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;
    let updated_jar = jar.add(auth_cookie);

    let res = two_fa_code_store.lock().await.remove_code(&email).await.map_err(|_| AuthAPIError::UnexpectedError)?;

    Ok((updated_jar, (StatusCode::OK, Json(Verify2FAResponse { message: "Ok".to_string() }))))
}


#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FAResponse {
    pub message: String,
}
