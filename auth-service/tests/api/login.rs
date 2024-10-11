use crate::helpers::TestApp;

#[tokio::test]
async fn login_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_login().await;

    assert_eq!(response.status().as_u16(), 200);
}
