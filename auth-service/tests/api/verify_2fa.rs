use crate::helpers::TestApp;


mod tests {
    use auth_service::{domain::email::Email, routes::TwoFactorAuthResponse, services::data_store::LoginAttemptId, utils::constants::JWT_COOKIE_NAME};

    use crate::helpers::get_random_email;

    use super::*;

    #[tokio::test]
    async fn should_return_422_if_malformed_input() {
        let app = TestApp::run().await;

        let test_cases = vec![
            serde_json::json!({
                "mail": "user@example.com",
                "loginAttemptId": "string",
                "2FACode": "string"
            }),
            serde_json::json!({
                "email": "user@example.com",
                "loghinAttemptId": "string",
                "2FACode": "string"
            }),
            serde_json::json!({
                "email": "user@example.com",
                "loghinAttemptId": "string",
                "2FAKCode": "string"
            }),
            serde_json::json!({
                "email": "user@example.com",
                "2FACode": "string"
            }),
            serde_json::json!({
                "loginAttemptId": "string",
                "2FACode": "string"
            }),
            serde_json::json!({
                "email": "user@example.com",
                "loginAttemptId": "string",
            }),
            serde_json::json!({}),
        ];

        for case in test_cases {
            let response = app.post_verify_2fa(&case).await;

            assert_eq!(response.status().as_u16(), 422);
        }
    }


    #[tokio::test]
    async fn should_return_400_if_invalid_input() {
        let app = TestApp::run().await;

        let test_cases = vec![
            serde_json::json!({
                "email": "userexample.com",
                "loginAttemptId": "string",
                "2FACode": "string"
            }),
            serde_json::json!({
                "email": "user@example.com",
                "loginAttemptId": "",
                "2FACode": "string"
            }),
            serde_json::json!({
                "email": "user@example.com",
                "loginAttemptId": "string",
                "2FACode": ""
            }),
        ];

        for case in test_cases {
            let response = app.post_verify_2fa(&case).await;

            assert_eq!(response.status().as_u16(), 400);
        }
    }



    #[tokio::test]
    async fn should_return_401_if_incorrect_credentials() {
        let app = TestApp::run().await;

        let email = get_random_email();

        // Create a user in the test database
        let signup_body = serde_json::json!({
            "email": email,
            "password": "password123",
            "requires2FA": true,
        });

        let response = app.post_signup(&signup_body).await;

        assert_eq!(response.status().as_u16(), 201);

        // Attempt to login with valid credentials
        let login_body = serde_json::json!({
            "email": email,
            "password": "password123",
        });

        let response = app.post_login(&login_body).await;

        assert_eq!(response.status().as_u16(), 206);

        let (login_attempt_id, code) = app.two_fa_code_store.lock().await.get_code(&Email::new(&email)).await.unwrap();
        let verify_2fa_body = serde_json::json!({
            "email": email,
            "loginAttemptId": LoginAttemptId::default().as_ref(),
            "2FACode": code.as_ref(),
        });

        let response = app.post_verify_2fa(&verify_2fa_body).await;

        assert_eq!(response.status().as_u16(), 401);
    }


    // #[tokio::test]
    // async fn should_return_401_if_old_code() {
    //     let app = TestApp::run().await;

    //     let email = get_random_email();

    //     let signup_body = serde_json::json!({
    //         "email": email,
    //         "password": "password123",
    //         "requires2FA": true
    //     });

    //     let response = app.post_signup(&signup_body).await;

    //     assert_eq!(response.status().as_u16(), 201);

    //     // First login call

    //     let login_body = serde_json::json!({
    //         "email": email,
    //         "password": "password123"
    //     });

    //     let response = app.post_login(&login_body).await;

    //     assert_eq!(response.status().as_u16(), 206);

    //     let response_body = response
    //         .json::<TwoFactorAuthResponse>()
    //         .await
    //         .expect("Could not deserialize response body to TwoFactorAuthResponse");

    //     assert_eq!(response_body.message, "2FA required".to_owned());
    //     assert!(!response_body.login_attempt_id.is_empty());

    //     let login_attempt_id = response_body.login_attempt_id;

    //     let (login_attempt_id, code) = app.two_fa_code_store.lock().await.get_code(&Email::new(&email)).await.unwrap();

    //     // Second login call

    //     let response = app.post_login(&login_body).await;

    //     assert_eq!(response.status().as_u16(), 206);

    //     // 2FA attempt with old login_attempt_id and code

    //     let request_body = serde_json::json!({
    //         "email": email,
    //         "loginAttemptId": login_attempt_id.as_ref(),
    //         "2FACode": code.as_ref()
    //     });

    //     let response = app.post_verify_2fa(&request_body).await;

    //     assert_eq!(response.status().as_u16(), 401);
    // }

    #[tokio::test]
    async fn should_return_200_if_correct_code() {
        let app = TestApp::run().await;

        let random_email = get_random_email();

        let signup_body = serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        });

        let response = app.post_signup(&signup_body).await;

        assert_eq!(response.status().as_u16(), 201);

        let login_body = serde_json::json!({
            "email": random_email,
            "password": "password123"
        });

        let response = app.post_login(&login_body).await;

        assert_eq!(response.status().as_u16(), 206);

        let response_body = response
            .json::<TwoFactorAuthResponse>()
            .await
            .expect("Could not deserialize response body to TwoFactorAuthResponse");

        assert_eq!(response_body.message, "2FA required".to_owned());
        assert!(!response_body.login_attempt_id.is_empty());

        let login_attempt_id = response_body.login_attempt_id;

        let (login_attempt_id, code) = app.two_fa_code_store.lock().await.get_code(&Email::new(&random_email)).await.unwrap();

        let request_body = serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": code.as_ref()
        });

        let response = app.post_verify_2fa(&request_body).await;

        assert_eq!(response.status().as_u16(), 200);

        let auth_cookie = response
            .cookies()
            .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
            .expect("No auth cookie found");

        assert!(!auth_cookie.value().is_empty());
    }


    #[tokio::test]
    async fn should_return_401_if_same_code_twice() {
        let app = TestApp::run().await;

        let random_email = get_random_email();

        let signup_body = serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        });

        let response = app.post_signup(&signup_body).await;

        assert_eq!(response.status().as_u16(), 201);

        let login_body = serde_json::json!({
            "email": random_email,
            "password": "password123"
        });

        let response = app.post_login(&login_body).await;

        assert_eq!(response.status().as_u16(), 206);

        let response_body = response
            .json::<TwoFactorAuthResponse>()
            .await
            .expect("Could not deserialize response body to TwoFactorAuthResponse");

        assert_eq!(response_body.message, "2FA required".to_owned());
        assert!(!response_body.login_attempt_id.is_empty());

        let login_attempt_id = response_body.login_attempt_id;

        let (login_attempt_id, code) = app.two_fa_code_store.lock().await.get_code(&Email::new(&random_email)).await.unwrap();

        let request_body = serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": code.as_ref()
        });

        let response = app.post_verify_2fa(&request_body).await;

        assert_eq!(response.status().as_u16(), 200);

        let auth_cookie = response
            .cookies()
            .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
            .expect("No auth cookie found");

        assert!(!auth_cookie.value().is_empty());

        let response = app.post_verify_2fa(&request_body).await;

        assert_eq!(response.status().as_u16(), 401);
    }
}
