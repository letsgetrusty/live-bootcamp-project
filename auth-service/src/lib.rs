use std::error::Error;

use axum::{routing::post, serve::Serve, Router};
use routes::{login, logout, signup, verify_2fa, verify_token};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

mod routes;

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let serve_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .fallback_service(serve_dir)
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
