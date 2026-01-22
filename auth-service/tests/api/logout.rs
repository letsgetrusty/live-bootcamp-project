use crate::helpers::TestApp;
use axum::body;


mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logout() {
        let app = TestApp::run().await;
        let response = app.post_logout().await;
        assert_eq!(response.status().as_u16(), 200);
    }
}
