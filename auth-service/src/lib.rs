use tokio::net::TcpListener;
use axum::{Router, routing::post, serve::Serve};
use tower_http::{services::{ServeDir, ServeFile}};

pub mod routes;
use routes::*;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self> {
        let assets_dir = ServeDir::new("assets")
        .not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token));
        let listener = TcpListener::bind(address).await?; 
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<()> {
        println!("Listening on {}", self.address);
        self.server.await?;
        Ok(())
    }
}



