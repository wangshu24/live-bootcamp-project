use super::helpers::{get_random_email, TestApp};

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

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    let test_cases = [serde_json::json!({
        "password": "password123",
        "requires2FA": true,
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
