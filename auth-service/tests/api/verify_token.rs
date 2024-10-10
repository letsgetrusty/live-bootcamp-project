use crate::helpers::TestApp;
#[tokio::test]
async fn post_verify_token_returns_200() {
    let app = TestApp::new().await;
    let response = app.post_verify_token().await;
    assert_eq!(response.status().as_u16(), 200);
}
