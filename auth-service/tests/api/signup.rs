use auth_service::{routes::SignupResponse, ErrorResponse};

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

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // The signup route returns a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters
    let app = TestApp::new().await;

    let input = [
        serde_json::json!({
            "email": "joebidenwhitehouse.gov",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "joebiden@whitehouse.gov",
            "password": "passw",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "",
            "password": "",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "joebidenwhitehouse.gov",
            "password": "passwo",
            "requires2FA": true
        }),
    ];

    // for test_case in input.iter() {
    //     let response = app.post_signup(test_case).await;
    //     assert_eq!(
    //         response.status().as_u16(),
    //         400,
    //         "Failed for input: {:?}",
    //         test_case
    //     );
    // }

    for i in input.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    // Creating the same account with the same email twice
    app.post_signup(&serde_json::json!({"email": "blanketjackson@neverland.com", "password": "password123", "requires2FA": false})).await;
    let response = app.post_signup(&serde_json::json!({"email": "blanketjackson@neverland.com", "password": "password123", "requires2FA": false})).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}


#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let response = app.post_signup(&serde_json::json!({"email": "joebiden@whitehouse.gov", "password": "password123", "requires2FA": true})).await;

    assert_eq!(response.status().as_u16(), 201, "Returned Status: {:?}", response.status().as_u16());

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}