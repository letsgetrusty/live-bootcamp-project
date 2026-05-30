use auth_service::{Application, app_state::{AppState, BannedTokenStoreType, EmailClientType, TwoFACodeStoreType, UserStoreType}, get_postgres_pool, services::email_client::EmailClient, utils::constants::{DATABASE_URL, prod}};
use sqlx::PgPool;


#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
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


async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
