use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let request = serde_json::json!({"e:": "p", "p": "p"});

    let response = app.post_login(&request).await;

    assert_eq!(
        response.status().as_u16(),
        422,
        "Failed for input: {:?}",
        request
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let request = serde_json::json!({"email": "p", "password": "p"});

    let response = app.post_login(&request).await;
    assert_eq!(response.status().as_u16(), 400);

        assert_eq!(
            response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid credentials".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.     
    let app = TestApp::new().await;

    // creating a test user
    app.post_signup(&serde_json::json!({"email": "joebiden@whitehouse.gov", "password": "password123", "requires2FA": true})).await;

    let request = serde_json::json!({"email": "joebiden@whitehouse.gov", "password": "password1234"});

    let response = app.post_login(&request).await;
    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
        .json::<ErrorResponse>()
        .await
        .expect("Could not deserialize response body to ErrorResponse")
        .error,
    "Passowrd is incorrect".to_owned());
}