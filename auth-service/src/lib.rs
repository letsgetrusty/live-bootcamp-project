use tokio::net::TcpListener;
use axum::{routing::get, Router, serve::Serve};
use tower_http::services::ServeDir;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self> {
        let assets_dir = ServeDir::new("assets");
        let router = Router::new()
            .fallback_service(assets_dir);
            // .route("/hello", get(hello_handler));
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
