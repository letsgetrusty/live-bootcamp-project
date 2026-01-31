use axum::{response::IntoResponse, http::StatusCode};

pub async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
