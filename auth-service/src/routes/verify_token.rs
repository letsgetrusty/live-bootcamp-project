use axum::{extract::State, http::StatusCode, Json};
use secrecy::Secret;
use serde::Deserialize;

use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};

#[tracing::instrument(name = "Verify token", skip_all)]
pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {
    match validate_token(&request.token, state.banned_token_store.clone()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: Secret<String>,
}
