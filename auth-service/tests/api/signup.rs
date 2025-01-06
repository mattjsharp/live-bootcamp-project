use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "password": "password123",
            "             email": &random_email,
            "require2FA": 15
        }),
        serde_json::json!({
            "passwoord": "password123",
            "email": &random_email,
            "requires2FA": true,
        }),
        serde_json::json!({
            "password": "password123",
            "email    ": &random_email,
            "emai": &random_email,
            "requires2FA": true
        }),
        serde_json::json!({}),
        serde_json::json!({
            "password      ": 8,
            "email": &random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": &random_email,
            "requires2FA ": false
        }),
        serde_json::json!({
            "password": "password123",
            "emaiil": &random_email,
            "requires_2FA": true
        }),
        serde_json::json!({
            "passworde": "password123",
            "password": "password123",
            "     email": &random_email,
            "password": "password23",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": &random_email,
            "email": &random_email,
            "email": &random_email,
            "email": &random_email,
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}