use crate::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

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

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}