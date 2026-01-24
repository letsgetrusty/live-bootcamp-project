use tokio::net::TcpListener;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post, serve::Serve};
use tower_http::{services::{ServeDir, ServeFile}};

pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;

use routes::*;
use domain::{error::AuthAPIError};
use app_state::AppState;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}


impl IntoResponse for AuthAPIError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}


pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self> {
        let assets_dir = ServeDir::new("assets")
        .not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state);
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
