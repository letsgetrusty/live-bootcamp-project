use crate::helpers::TestApp;


mod tests {
    use super::*;

    #[tokio::test]
    async fn root_returns_auth_ui() {
        let app = TestApp::run().await;
        let response = app.get_root().await;
        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(response.headers().get("content-Type").unwrap(), "text/html");
    }
}
