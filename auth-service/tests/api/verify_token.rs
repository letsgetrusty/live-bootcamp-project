use crate::helpers::TestApp;


mod tests {
    use super::*;

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
