use auth_service::{Application, app_state::{AppState, BannedTokenStoreType, EmailClientType, TwoFACodeStoreType, UserStoreType}, services::email_client::EmailClient, utils::constants::prod};


#[tokio::main]
async fn main() {
    let user_store = UserStoreType::default();
    let banned_token_store = BannedTokenStoreType::default();
    let two_fa_code_store = TwoFACodeStoreType::default();
    let email_client = EmailClientType::default();
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    ); // optionally, if builder pattern enabled: AppState::with_store(hashmap_store).build();

    let app = Application::build(app_state, prod::APP_ADDRESS).await.expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
