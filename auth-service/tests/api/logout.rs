use crate::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);

    let cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(cookie.is_none());

    assert_eq!(
        response
        .json::<ErrorResponse>()
        .await
        .expect("Could not deserialize response body to ErrorResponse")
        .error,
        "Missing Token".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);

    let cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(cookie.is_none());

    assert_eq!(
        response
        .json::<ErrorResponse>()
        .await
        .expect("Could not deserialize response body to ErrorResponse")
        .error,
        "Invalid Token".to_owned()
    );
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let email = get_random_email();

    let signup_response = app.post_signup(&serde_json::json!({
        "email": email,
        "password": "password123",
        "requires2FA": false
    })).await;

    assert_eq!(signup_response.status().as_u16(), 201, "Failed to signup");

    let login_response = app.post_login(&serde_json::json!({
        "email": email,
        "password": "password123",
    })).await;

    assert_eq!(login_response.status().as_u16(), 200, "Failed to login");

    let cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("Cookie not found");

    let token = cookie.value();

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(auth_cookie.value().is_empty());

    let banned_token_store = app.banned_token_store.read().await;
    let contains_token = banned_token_store
        .contains_token(token)
        .await
        .expect("Failed to check if token is banned");

    assert!(contains_token);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let email = get_random_email();

    let signup_response = app.post_signup(&serde_json::json!({
        "email": email,
        "password": "password123",
        "requires2FA": false
    })).await;

    // Signing up a user
    assert_eq!(signup_response.status().as_u16(), 201, "Failed to signup");

    let login_response = app.post_login(&serde_json::json!({
        "email": email,
        "password": "password123",
    })).await;

    // logging in with new user
    assert_eq!(login_response.status().as_u16(), 200, "Failed to login");

    let cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("Cookie not found");

    // Checking if cookie is present
    assert!(!cookie.value().is_empty(), "Cookie is empty");

    let logout_response = app.post_logout().await;

    // logging out user
    assert_eq!(logout_response.status().as_u16(), 200);

    let cookie = logout_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("Cookie not found");

    // Checking if cookie is now empty
    assert!(cookie.value().is_empty(), "Cookie not removed");

    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 400);

    assert_eq!(
        logout_response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Missing Token".to_owned()
    );

}