use axum::{http::StatusCode, response::IntoResponse};

pub async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
