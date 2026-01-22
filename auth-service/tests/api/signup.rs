use crate::helpers::TestApp;
use axum::body;


mod tests {
    use super::*;

    #[tokio::test]
    async fn test_signup() {
        let app = TestApp::run().await;

        let body = serde_json::json!({
            "email": "user@example.com",
            "password": "password",
            "requires2FA": true
        });
        let response = app.post_signup(&body).await;
        assert_eq!(response.status().as_u16(), 200);
    }
}
