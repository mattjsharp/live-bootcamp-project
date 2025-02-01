use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode},
    utils::auth::generate_auth_cookie,
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let user_store = state.user_store.write().await;

    let email = Email::parse(&request.email);
    let password = Password::parse(&request.password);

    let email = match email {
        Err(_e) => return (jar, Err(AuthAPIError::InvalidCredentials)),
        Ok(val) => val,
    };

    let password = match password {
        Err(_e) => return (jar, Err(AuthAPIError::InvalidCredentials)),
        Ok(val) => val,
    };

    let user = match user_store.get_user(&email).await {
        Err(_e) => return (jar, Err(AuthAPIError::InvalidCredentials)),
        Ok(val) => val,
    };

    if user.password != password {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    match state.two_fa_code_store.write().await.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await {
        Ok(_) => (),
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError))
    };

    if let Err(_) = state.email_client.write().await.send_email(email, "Your 2FA Code", two_fa_code.as_ref()).await {
        return (jar, Err(AuthAPIError::UnexpectedError))
    }

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar, 
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>
) {
    let auth_cookie = match generate_auth_cookie(&email) {
        Err(_e) => return (jar, Err(AuthAPIError::UnexpectedError)),
        Ok(val) => val,
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}