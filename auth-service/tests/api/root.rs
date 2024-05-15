use test_helpers::api_test;

use crate::helpers::TestApp;

#[api_test]
async fn root_returns_auth_ui() {
    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
