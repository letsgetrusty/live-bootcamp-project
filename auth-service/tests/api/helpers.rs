use auth_service::Application;
use reqwest;



pub struct TestApp {
    pub address: String,
    pub client: reqwest::Client,
}

impl TestApp {
    pub async fn run() -> Self {
        let app = Application::build("127.0.0.1:0").await.expect("Failed to build application");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());
        Self { address, client: reqwest::Client::new() }
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

    pub async fn post_login(&self, body: &serde_json::Value) -> reqwest::Response {
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

    pub async fn verify_token(&self, body: &serde_json::Value) -> reqwest::Response {
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
