use auth_service::{Application, app_state::{AppState, UserStoreType}};


#[tokio::main]
async fn main() {
    let user_store = UserStoreType::default();
    let app_state = AppState::new(user_store); // optionally, if builder pattern enabled: AppState::with_store(hashmap_store).build();

    let app = Application::build(app_state, "0.0.0.0:3000").await.expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
