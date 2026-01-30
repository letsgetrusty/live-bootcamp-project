use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::{email::Email, error::AuthAPIError, password::Password}, services::data_store::{LoginAttemptId, TwoFACode}, utils::auth::generate_auth_cookie};


pub async fn login(
    State(_state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = request.email;
    let password = request.password;

    // early return AuthAPIError::InvalidCredentials if email is empty or does not contain "@" symbol or password is less than 8 characters long
    let email = Email::parse(email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    // let user_store = &_state.user_store;
    // match user_store.get_user(email.clone()).await {
    //     Ok(user) => {
    //         if user.password != password {
    //             return Err(AuthAPIError::IncorrectCredentials);
    //         }
    //     }
    //     Err(_) => {
    //         return Err(AuthAPIError::IncorrectCredentials);
    //     }
    // }

    // let auth_cookie = generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

    // let updated_jar = jar.add(auth_cookie);

    // return Ok((updated_jar, StatusCode::OK.into_response()));

    // let user = &_state.user_store.get_user(email.clone()).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;
    let user_store = &_state.user_store;
    let user = match user_store.get_user(email.clone()).await {
        Ok(user) => {
            if user.password != password {
                return Err(AuthAPIError::IncorrectCredentials);
            }
            else { user }
        }
        Err(_) => {
            return Err(AuthAPIError::IncorrectCredentials);
        }
    };

    match user.requires_2fa {
        // We are now passing `&user.email` and `&state` to `handle_2fa`
        true => handle_2fa(&user.email, &_state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}


async fn handle_2fa(
    email: &Email, // New!
    state: &AppState, // New!
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    // First, we must generate a new random login attempt ID and 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let code = TwoFACode::default();

    state.two_fa_code_store.lock().await.add_code(email.to_owned(), login_attempt_id.clone(), code).await.map_err(|_| AuthAPIError::UnexpectedError)?;

    // Finally, we need to return the login attempt ID to the client
    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_string(), // Add the generated login attempt ID
    }));

    Ok((
        jar,
        (
            StatusCode::PARTIAL_CONTENT,
            response
        )
    ))
}


async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    let auth_cookie = generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);
    Ok((updated_jar, (StatusCode::OK, Json(LoginResponse::RegularAuth))))
}


#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}


// #[derive(Serialize, PartialEq, Debug, Deserialize)]
// pub struct LoginResponse {
//     pub message: String,
// }


#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
