use axum::{response::IntoResponse, http::StatusCode};

pub async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
