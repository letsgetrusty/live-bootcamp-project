use crate::helpers::TestApp;


mod tests {
    use auth_service::utils::constants::JWT_COOKIE_NAME;
    use axum_extra::extract::cookie;
    use reqwest::Url;

    use crate::helpers::get_random_email;

    use super::*;

    #[tokio::test]
    async fn should_return_400_if_jwt_cookie_missing() {
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

        let response = app.post_logout().await;

        assert_eq!(response.status().as_u16(), 400);
    }

    #[tokio::test]
    async fn should_return_401_if_invalid_token() {
        let app = TestApp::run().await;

        app.cookie_jar.add_cookie_str(
            &format!(
                "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
                JWT_COOKIE_NAME,
            ),
            &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
        );

        let response = app.post_logout().await;

        assert_eq!(response.status().as_u16(), 401);
    }

    #[tokio::test]
    async fn should_return_200_if_valid_jwt_cookie() {
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

        let response = app.post_logout().await;

        assert_eq!(response.status().as_u16(), 200);

        assert_eq!(app.banned_token_store.check_token(&auth_cookie.value()).await.unwrap(), true);
    }

    #[tokio::test]
    async fn should_return_400_if_logout_called_twice_in_a_row() {
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

        let response = app.post_logout().await;

        assert_eq!(response.status().as_u16(), 200);

        let response = app.post_logout().await;

        assert_eq!(response.status().as_u16(), 400);
    }
}
