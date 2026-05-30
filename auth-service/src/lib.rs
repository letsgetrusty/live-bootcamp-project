use tokio::net::TcpListener;
use axum::{Json, Router, http::{Method, StatusCode}, response::{IntoResponse, Response}, routing::post, serve::Serve};
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod routes;
pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;

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
            AuthAPIError::MalformedRequest => (StatusCode::UNPROCESSABLE_ENTITY, "Malformed request"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token")
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
        let allowed_origins = [
            "http://localhost:8000".parse()?,
        ];

        let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_origin(allowed_origins);

        let assets_dir = ServeDir::new("assets")
        .not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);
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



pub async fn get_postgres_pool(url: &str) -> std::result::Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}
