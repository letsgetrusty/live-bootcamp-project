use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{app_state::AppState, domain::error::AuthAPIError, utils::auth::validate_token};


pub async fn verify_token(
    State(_state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    validate_token(&request.token, _state.banned_token_store.clone()).await.map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(StatusCode::OK.into_response())
}


#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}
