use super::helpers::TestApp;
use crate::routes::SignupRequest;

#[tokio::test]
async fn signup_should_return_200() {
    let app = TestApp::new().await;

    let body = SignupRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        requires_2fa: false,
    };

    let response = app.post_signup(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
