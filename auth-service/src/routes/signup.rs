use axum::http::StatusCode;
use axum::response::IntoResponse;
pub async fn post_signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
