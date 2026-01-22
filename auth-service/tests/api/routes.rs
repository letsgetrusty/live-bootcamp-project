use crate::helpers::TestApp;


mod tests {
    use axum::body;

    use super::*;

    #[tokio::test]
    async fn root_returns_auth_ui() {
        let app = TestApp::run().await;
        let response = app.get_root().await;
        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(response.headers().get("content-Type").unwrap(), "text/html");
    }

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

    #[tokio::test]
    async fn test_login() {
        let app = TestApp::run().await;
        let body = serde_json::json!({
            "email": "user@example.com",
            "password": "password"
        });
        let response = app.post_login(&body).await;
        assert_eq!(response.status().as_u16(), 200);
    }

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

    #[tokio::test]
    async fn test_logout() {
        let app = TestApp::run().await;
        let response = app.post_logout().await;
        assert_eq!(response.status().as_u16(), 200);
    }

    #[tokio::test]
    async fn test_verify_token() {
        let app = TestApp::run().await;
        let body = serde_json::json!({
            "token": "some_valid_token"
        });
        let response = app.verify_token(&body).await;
        assert_eq!(response.status().as_u16(), 200);
    }
}
