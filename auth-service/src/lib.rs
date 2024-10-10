use std::error::Error;
use axum::{serve::Serve, response::Html, routing::get, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", get(post_signup))
            .route("/login", get(post_login))
            .route("/logout", get(post_logout))
            .route("/verify-2fa", get(post_verify_2fa))
            .route("/verify-token", get(post_verify_token));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn post_signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
async fn post_login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
async fn post_logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
async fn post_verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
async fn post_verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}