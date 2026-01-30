use std::sync::Arc;

use auth_service::{Application, app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType, UserStoreType}, utils::constants::test};
use reqwest::{self, Client, cookie::Jar};


pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub client: reqwest::Client,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl TestApp {
    pub async fn run() -> Self {
        let user_store = UserStoreType::default();
        let banned_token_store = BannedTokenStoreType::default();
        let two_fa_code_store = TwoFACodeStoreType::default();

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
        );

        let app = Application::build(app_state, test::APP_ADDRESS).await.expect("Failed to build application");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = Client::builder()
        .cookie_provider(cookie_jar.clone())
        .build()
        .unwrap();

        Self { address, cookie_jar, client: http_client, banned_token_store, two_fa_code_store }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response 
    where Body: serde::Serialize
    {
        self.client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response 
    where Body: serde::Serialize
    {
        self.client
            .post(format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self,) -> reqwest::Response {
        self.client
            .post(format!("{}/logout", &self.address))
            // token is in header in real use, but for testing we can skip it
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa(&self, body: &serde_json::Value) -> reqwest::Response {
        self.client
            .post(format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where Body: serde::Serialize {
        self.client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}


pub fn get_random_email() -> String {
    let uuid = uuid::Uuid::new_v4();
    format!("{}@example.com", uuid)
}
