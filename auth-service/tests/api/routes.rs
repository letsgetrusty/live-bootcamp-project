use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_login().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
}
