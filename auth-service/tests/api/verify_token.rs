use crate::helpers::TestApp;


mod tests {
    use auth_service::utils::constants::JWT_COOKIE_NAME;
    use reqwest::{Url, cookie::CookieStore};

    use crate::helpers::get_random_email;

    use super::*;

    #[tokio::test]
    async fn should_return_422_if_malformed_input() {
        let app = TestApp::run().await;

        let test_cases = vec![
            serde_json::json!({
                "toghen": "mytoken",
            }),
            serde_json::json!({}),
        ];

        for case in test_cases {
            let response = app.post_login(&case).await;

            assert_eq!(response.status().as_u16(), 422);
        }
    }

    #[tokio::test]
    async fn should_return_200_if_valid_token() {
        let app = TestApp::run().await;

        let email = get_random_email();

        // Create a user in the test database
        let signup_body = serde_json::json!({
            "email": email,
            "password": "password123",
            "requires2FA": false,
        });

        let response = app.post_signup(&signup_body).await;

        assert_eq!(response.status().as_u16(), 201);

        // Attempt to login with valid credentials
        let login_body = serde_json::json!({
            "email": email,
            "password": "password123",
        });

        let response = app.post_login(&login_body).await;

        assert_eq!(response.status().as_u16(), 200);

        let auth_cookie = response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .expect("no auth cookie found");

        let token = auth_cookie.value();

        let verify_token_body = serde_json::json!({
            "token": token
        });

        let response = app.post_verify_token(&verify_token_body).await;

        assert_eq!(response.status().as_u16(), 200);
    }

    #[tokio::test]
    async fn should_return_401_if_invalid_token() {
        let app = TestApp::run().await;

        let verify_token_body = serde_json::json!({
            "token": "invalid"
        });

        let response = app.post_verify_token(&verify_token_body).await;

        assert_eq!(response.status().as_u16(), 401);
    }

    #[tokio::test]
    async fn should_return_401_if_banned_token() {
        let app = TestApp::run().await;

        let email = get_random_email();

        // Create a user in the test database
        let signup_body = serde_json::json!({
            "email": email,
            "password": "password123",
            "requires2FA": false,
        });

        let response = app.post_signup(&signup_body).await;

        assert_eq!(response.status().as_u16(), 201);

        // Attempt to login with valid credentials
        let login_body = serde_json::json!({
            "email": email,
            "password": "password123",
        });

        let response = app.post_login(&login_body).await;

        assert_eq!(response.status().as_u16(), 200);

        let auth_cookie = response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .expect("no auth cookie found");

        let token = auth_cookie.value();

        app.banned_token_store.store_token(&token).await.unwrap();

        let verify_token_body = serde_json::json!({
            "token": token
        });

        let response = app.post_verify_token(&verify_token_body).await;

        assert_eq!(response.status().as_u16(), 401);
    }
}
