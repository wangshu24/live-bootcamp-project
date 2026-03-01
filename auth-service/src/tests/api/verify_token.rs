use super::helpers::TestApp;
#[tokio::test]
pub async fn verify_token_should_return_200() {
    let app = TestApp::new().await;

    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
}