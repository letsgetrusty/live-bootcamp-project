pub mod routes;

use std::error::Error;
use axum::{serve::Serve, routing::get, routing::post, Router};
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
            .route("/signup", post(routes::post_signup))
            .route("/login", post(routes::post_login))
            .route("/logout", post(routes::post_logout))
            .route("/verify-2fa", post(routes::post_verify_2fa))
            .route("/verify-token", post(routes::post_verify_token));

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
