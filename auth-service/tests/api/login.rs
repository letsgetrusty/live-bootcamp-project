use crate::helpers::TestApp;


mod tests {
    use super::*;

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
}
