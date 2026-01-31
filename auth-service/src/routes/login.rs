use axum::{response::IntoResponse, http::StatusCode};

pub async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
