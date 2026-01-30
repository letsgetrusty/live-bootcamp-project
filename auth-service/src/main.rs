use auth_service::{Application, app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType, UserStoreType}, utils::constants::prod};


#[tokio::main]
async fn main() {
    let user_store = UserStoreType::default();
    let banned_token_store = BannedTokenStoreType::default();
    let two_fa_code_store = TwoFACodeStoreType::default();
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
    ); // optionally, if builder pattern enabled: AppState::with_store(hashmap_store).build();

    let app = Application::build(app_state, prod::APP_ADDRESS).await.expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
