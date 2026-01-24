use crate::helpers::TestApp;


mod tests {
    use auth_service::{ErrorResponse, routes::SignupResponse};
    use axum::Json;
    use serde_json::json;

    use crate::helpers::get_random_email;

    use super::*;

    #[tokio::test]
    async fn should_return_422_if_malformed_input() {
        let app = TestApp::run().await;

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

    #[tokio::test]
    async fn should_return_201_if_valid_input() {
        let app = TestApp::run().await;

        let email = get_random_email();

        let body = json!({
            "email": email,
            "password": "securepassword",
            "requires2FA": true
        });

        let response = app.post_signup(&body).await;
        assert_eq!(response.status().as_u16(), 201);

        let expected_response = Json(SignupResponse {
            message: "User created successfully!".to_owned(),
        });

        assert_eq!(
            response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
            expected_response.0
        );
    }

    #[tokio::test]
    async fn should_return_400_if_invalid_input() {
        // the input is considered invalid if:
        // - the email is empty or does not contain an "@" symbol
        // - the password is less than 8 characters long

        // create an array of invalid inputs, then iterate over them and assert that the response status is 400
        let app = TestApp::run().await;
        let test_cases = [
            json!({
                "email": "",
                "password": "securepassword",
                "requires2FA": true
            }),
            json!({
                "email": "invalidemail.com",
                "password": "securepassword",
                "requires2FA": true
            }),
            json!({
                "email": get_random_email(),
                "password": "short",
                "requires2FA": true
            }),
        ];

        for body in test_cases {
            let response = app.post_signup(&body).await;
            assert_eq!(response.status().as_u16(), 400, "Failed for body: {}", body);

            assert_eq!(
                response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to Value")
                .error,
                "Invalid credentials".to_string(),
            );
        }
   
    }

    #[tokio::test]
    async fn should_return_409_if_email_already_exists() {
        let app = TestApp::run().await;

        let email = get_random_email();

        let body = json!({
            "email": email,
            "password": "securepassword",
            "requires2FA": true
        });

        // first signup attempt should succeed
        let response = app.post_signup(&body).await;
        assert_eq!(response.status().as_u16(), 201);

        let expected_response = Json(SignupResponse {
            message: "User created successfully!".to_owned(),
        });

        assert_eq!(
            response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
            expected_response.0
        );

        // second signup attempt with the same email should fail
        let response = app.post_signup(&body).await;
        assert_eq!(response.status().as_u16(), 409);

        assert_eq!(
            response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to Value")
            .error,
            "User already exists".to_string(),
        );
    }
}