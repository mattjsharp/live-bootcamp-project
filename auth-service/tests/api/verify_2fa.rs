use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore}, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME, ErrorResponse
};
use uuid::Uuid;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let request = serde_json::json!({"e:": "p", "p": "p"});

    let response = app.post_verify_2fa(&request).await;

    assert_eq!(
        response.status().as_u16(),
        422,
        "Failed for input: {:?}",
        request
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let valid_uuid = Uuid::new_v4().to_string();

    let request = serde_json::json!({
        "email": "notvalid", // Invalid Email
        "loginAttemptId": &valid_uuid,
        "2FACode": "123456"
      });

    let response = app.post_verify_2fa(&request).await;
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid credentials".to_owned()
    );

    let request = serde_json::json!({
        "email": "joebiden@whitehouse.gov",
        "loginAttemptId": "invalid", // Invalid UUID
        "2FACode": "123456"
      });

    let response = app.post_verify_2fa(&request).await;
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid credentials".to_owned()
    );

    let request = serde_json::json!({
        "email": "joebiden@whitehouse.gov",
        "loginAttemptId": &valid_uuid,
        "2FACode": "12345" // Invalid 2FACode
      });

    let response = app.post_verify_2fa(&request).await;
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

// #[tokio::test]
// async fn should_return_401_if_incorrect_credentials() {
//     let app = TestApp::new().await;

//     let random_email = get_random_email();

//     let signup_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": true
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201); // Signing up an account with 2FA

//     let login_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//     });

//     let response = app.post_login(&login_body).await;

//     assert_eq!(response.status().as_u16(), 206); // Logging in with that account

//     let json_body = response
//         .json::<TwoFactorAuthResponse>()
//         .await
//         .expect("Could not deserialize response body to TwoFactorAuthResponse");

//     assert_eq!(json_body.message, "2FA required".to_owned());

//     let user_email = &Email::parse(&random_email).expect("Invalid Email");
//     let binding = app
//         .two_fa_code_store
//         .read()
//         .await;
//     let (login_attempt_id, code) = 
//         binding
//         .get_code(&user_email).await
//         .expect("Failed to get code");

//     assert_eq!(json_body.login_attempt_id, login_attempt_id.0);

//     let request = serde_json::json!({
//         "email": "invalid@email.com",
//         "loginAttemptId": &login_attempt_id.0,
//         "2FACode": &code.0
//       });

//     let response = app.post_verify_2fa(&request).await;
//     assert_eq!(response.status().as_u16(), 401);
// }

// #[tokio::test]
// async fn should_return_401_if_old_code() {
//     // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail. 
//     let app = TestApp::new().await;
    
//     let random_email = get_random_email();
//     let user_email = &Email::parse(&random_email).expect("Invalid Email");

//     let signup_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": true
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201); // Signing up an account with 2FA

//     let login_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//     });

//     let response = app.post_login(&login_body).await;

//     assert_eq!(response.status().as_u16(), 206); // Logging in with that account

//     let json_body = response
//         .json::<TwoFactorAuthResponse>()
//         .await
//         .expect("Could not deserialize response body to TwoFactorAuthResponse");

//     assert_eq!(json_body.message, "2FA required".to_owned());

//     let first_code = 
//     app
//     .two_fa_code_store
//     .write()
//     .await
//     .get_code(&user_email)
//     .await
//     .expect("Failed to get code")
//     .1.0;

//     let response = app.post_login(&login_body).await;

//     assert_eq!(response.status().as_u16(), 206); // Second Login

//     let json_body = response
//         .json::<TwoFactorAuthResponse>()
//         .await
//         .expect("Could not deserialize response body to TwoFactorAuthResponse");

//     assert_eq!(json_body.message, "2FA required".to_owned());

//     let binding = app
//         .two_fa_code_store
//         .write()
//         .await;
//     let (login_attempt_id, code) = 
//         binding
//         .get_code(&user_email).await
//         .expect("Failed to get code");

//     assert_eq!(json_body.login_attempt_id, login_attempt_id.0);

//     let request = serde_json::json!({
//         "email": &random_email,
//         "loginAttemptId": &login_attempt_id.0,
//         "2FACode": &first_code
//       });

//     let response = app.post_verify_2fa(&request).await;
//     assert_eq!(response.status().as_u16(), 401);
// }

// #[tokio::test]
// async fn should_return_200_if_correct_code() {
//     let app = TestApp::new().await;

//     let random_email = get_random_email();

//     let signup_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": true
//     });

//     let response = app.post_signup(&signup_body).await;

//     assert_eq!(response.status().as_u16(), 201); // Signing up an account with 2FA

//     let login_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//     });

//     let response = app.post_login(&login_body).await;

//     assert_eq!(response.status().as_u16(), 206); // Logging in with that account

//     let json_body = response
//         .json::<TwoFactorAuthResponse>()
//         .await
//         .expect("Could not deserialize response body to TwoFactorAuthResponse");

//     assert_eq!(json_body.message, "2FA required".to_owned());

//     let user_email = &Email::parse(&random_email).expect("Invalid Email");
//     let (login_attempt_id, code) = 
//         app
//         .two_fa_code_store
//         .read()
//         .await
//         .get_code(&user_email).await
//         .expect("Failed to get code");

//     assert_eq!(json_body.login_attempt_id, login_attempt_id.0);

//     let request = serde_json::json!({
//         "email": &random_email,
//         "loginAttemptId": &login_attempt_id.0,
//         "2FACode": &code.0
//       });

//     let response = app.post_verify_2fa(&request).await;
//     assert_eq!(response.status().as_u16(), 200);
// }

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());
    let login_attempt_id = response_body.login_attempt_id;
    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email.clone()).unwrap())
        .await
        .unwrap();
    let code = code_tuple.1.as_ref();
    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });
    let response = app.post_verify_2fa(&request_body).await;
    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    // --------------------------
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());
    let login_attempt_id = response_body.login_attempt_id;
    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email.clone()).unwrap())
        .await
        .unwrap();
    let two_fa_code = code_tuple.1.as_ref();
    // --------------------------
    let incorrect_email = get_random_email();
    let incorrect_login_attempt_id = LoginAttemptId::default().as_ref().to_owned();
    let incorrect_two_fa_code = TwoFACode::default().as_ref().to_owned();
    let test_cases = vec![
        (
            incorrect_email.as_str(),
            login_attempt_id.as_str(),
            two_fa_code,
        ),
        (
            random_email.as_str(),
            incorrect_login_attempt_id.as_str(),
            two_fa_code,
        ),
        (
            random_email.as_str(),
            login_attempt_id.as_str(),
            incorrect_two_fa_code.as_ref(),
        ),
    ];
    for (email, login_attempt_id, code) in test_cases {
        let request_body = serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        });
        let response = app.post_verify_2fa(&request_body).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            request_body
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Passowrd is incorrect".to_owned()
        );
    }
}

async fn should_return_401_if_old_code() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    // First login call
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());
    let login_attempt_id = response_body.login_attempt_id;
    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email.clone()).unwrap())
        .await
        .unwrap();
    let code = code_tuple.1.as_ref();
    // Second login call
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    // 2FA attempt with old login_attempt_id and code
    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });
    let response = app.post_verify_2fa(&request_body).await;
    assert_eq!(response.status().as_u16(), 401);
}
#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);
    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());
    let login_attempt_id = response_body.login_attempt_id;
    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email.clone()).unwrap())
        .await
        .unwrap();
    let code = code_tuple.1.as_ref();
    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });
    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 200);
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No cookie found");
    assert!(!auth_cookie.value().is_empty());
    let response = app.post_verify_2fa(&request_body).await;
    assert_eq!(response.status().as_u16(), 401);
}