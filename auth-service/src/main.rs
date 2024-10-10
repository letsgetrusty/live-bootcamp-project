use auth_service::Application;
use axum::response::Html;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello from the world of Rust.</h1>")
}
