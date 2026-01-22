use crate::helpers::TestApp;
use axum::body;


mod tests {
    use super::*;

    #[tokio::test]
    async fn test_verify_2fa() {
        let app = TestApp::run().await;
        let body = serde_json::json!({
            "email": "user@example.com",
            "loginAttemptId": "1",
            "2FACode": "123456"
        });
        let response = app.verify_2fa(&body).await;
        assert_eq!(response.status().as_u16(), 200);
    }
}
