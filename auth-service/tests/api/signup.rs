use crate::helpers::TestApp;
use axum::body;


mod tests {
    use serde_json::json;

    use crate::helpers::get_random_email;

    use super::*;

    #[tokio::test]
    async fn should_return_422_if_malformed_input() {
        let app = TestApp::run().await;

        let email = get_random_email();

        let test_cases = [
            json!({
                "password": "password",
                "requires2FA": true
            } ),
        ];

        for body in test_cases {
            let response = app.post_signup(&body).await;
            assert_eq!(response.status().as_u16(), 422, "Failed for body: {}", body);
        }
    }
}
