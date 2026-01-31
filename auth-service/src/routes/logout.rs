use axum::{response::IntoResponse, http::StatusCode};

pub async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
