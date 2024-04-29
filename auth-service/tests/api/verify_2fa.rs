use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
    ErrorResponse,
};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;

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

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
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
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let login_attempt_id = LoginAttemptId::default().as_ref().to_owned();
    let two_fa_code = TwoFACode::default().as_ref().to_owned();

    let test_cases = vec![
        (
            "invalid_email",
            login_attempt_id.as_str(),
            two_fa_code.as_str(),
        ),
        (
            random_email.as_str(),
            "invalid_login_attempt_id",
            two_fa_code.as_str(),
        ),
        (
            random_email.as_str(),
            login_attempt_id.as_str(),
            "invalid_two_fa_code",
        ),
        ("", "", ""),
    ];

    for (email, login_attempt_id, code) in test_cases {
        let request_body = serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        });

        let response = app.post_verify_2fa(&request_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            request_body
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // --------------------------

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

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let two_fa_code = code_tuple.1.as_ref();

    // --------------------------

    let incorrect_email = get_random_email();
    let incorrect_login_attempt_id = LoginAttemptId::default().as_ref().to_owned();
    let incorrect_two_fa_code = TwoFACode::default().as_ref().to_owned();

    let test_cases = vec![
        (
            incorrect_email.as_str(),
            login_attempt_id.as_str(),
            two_fa_code,
        ),
        (
            random_email.as_str(),
            incorrect_login_attempt_id.as_str(),
            two_fa_code,
        ),
        (
            random_email.as_str(),
            login_attempt_id.as_str(),
            incorrect_two_fa_code.as_ref(),
        ),
    ];

    for (email, login_attempt_id, code) in test_cases {
        let request_body = serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        });

        let response = app.post_verify_2fa(&request_body).await;

        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            request_body
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // First login call

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

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    // Second login call

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // 2FA attempt with old login_attempt_id and code

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;

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

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
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

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let login_attempt_id = LoginAttemptId::default().as_ref().to_owned();

    let test_cases = [
        serde_json::json!({
            "2FACode": "123456",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "loginAttemptId": login_attempt_id,
        }),
        serde_json::json!({
            "2FACode": "123456",
            "email": random_email,
        }),
        serde_json::json!({
            "2FACode": "123456",
            "loginAttemptId": login_attempt_id,
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
