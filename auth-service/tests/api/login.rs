use crate::helpers::TestApp;


mod tests {
    use auth_service::utils::constants::JWT_COOKIE_NAME;

    use crate::helpers::get_random_email;

    use super::*;

    #[tokio::test]
    async fn should_return_422_if_malformed_credentials() {
        let app = TestApp::run().await;

        let test_cases = vec![
            serde_json::json!({
                "mail": "user@example.com",
                "password": "password123",
            }),
            serde_json::json!({
                "email": "user@example.com",
                "passgword": "password123",
            }),
            serde_json::json!({
                "email": "user@example.com",
            }),
            serde_json::json!({
                "password": "password123",
            }),
            serde_json::json!({}),
        ];

        for case in test_cases {
            let response = app.post_login(&case).await;

            assert_eq!(response.status().as_u16(), 422);
        }
    }

    #[tokio::test]
    async fn should_return_400_if_invalid_input() {
        let app = TestApp::run().await;

        let test_cases = vec![
            serde_json::json!({
                "email": "invalid-email",
                "password": "password123",
            }),
            serde_json::json!({
                "email": "user@example.com",
                "password": "pass",
            }),
            serde_json::json!({
                "email": "",
                "password": "",
            }),
        ];

        for case in test_cases {
            let response = app.post_login(&case).await;

            assert_eq!(response.status().as_u16(), 400);
        }
    }

    #[tokio::test]
    async fn should_return_401_if_incorrect_credentials() {
        let app = TestApp::run().await;

        // Create a user in the test database
        let request_body = serde_json::json!({
            "email": "user@example.com",
            "password": "password123",
            "requires2FA": false,
        });

        let response = app.post_signup(&request_body).await;

        assert_eq!(response.status().as_u16(), 201);

        // Attempt to login with incorrect password
        let wrong_password_body = serde_json::json!({
            "email": "user@example.com",
            "password": "wrongpassword",
        });

        let response = app.post_login(&wrong_password_body).await;

        assert_eq!(response.status().as_u16(), 401);

        // Attempt to login with non-existent email
        let non_existent_email_body = serde_json::json!({
            "email": "foo@example.com",
            "password": "password123",
        });

        let response = app.post_login(&non_existent_email_body).await;

        assert_eq!(response.status().as_u16(), 401);
    }

    #[tokio::test]
    async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
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
    }
}
