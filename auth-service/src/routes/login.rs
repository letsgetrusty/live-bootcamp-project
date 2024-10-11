use axum::{http::StatusCode, response::IntoResponse};

pub async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
