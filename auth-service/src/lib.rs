use tokio::net::TcpListener;
use axum::{Router, response::IntoResponse, routing::{get, post}, serve::Serve, http::StatusCode};
use tower_http::{services::{ServeDir, ServeFile}};


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


async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}


async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}


async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}


async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}


async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
