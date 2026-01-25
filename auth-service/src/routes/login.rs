use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{email::Email, error::AuthAPIError, password::Password}};


pub async fn login(
    State(_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    // early return AuthAPIError::InvalidCredentials if email is empty or does not contain "@" symbol or password is less than 8 characters long
    let email = Email::parse(email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &_state.user_store;
    match user_store.get_user(email).await {
        Ok(user) => {
            if user.password != password {
                return Err(AuthAPIError::IncorrectCredentials);
            }
        }
        Err(_) => {
            return Err(AuthAPIError::IncorrectCredentials);
        }
    }

    Ok(StatusCode::OK.into_response())
}


#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}


#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct LoginResponse {
    pub message: String,
}
