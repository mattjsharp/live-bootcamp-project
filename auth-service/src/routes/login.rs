use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password, User, UserStoreError}, utils::auth::generate_auth_cookie};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar, // New!
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let user_store = state.user_store.write().await;
    
    let email = Email::parse(&request.email);
    let password = Password::parse(&request.password);
    let user: Result<User, UserStoreError>;

    let email = 
        match email { 
            Err(e) => { return (jar, Err(AuthAPIError::InvalidCredentials)) }
            Ok(val) => val
        };
    
    let password = 
        match password { 
            Err(e) => { return (jar, Err(AuthAPIError::InvalidCredentials)) }
            Ok(val) => val
        };

    let user = 
        match user_store.get_user(&email).await { 
            Err(e) => { return (jar, Err(AuthAPIError::InvalidCredentials)) }
            Ok(val) => val
        };

    if user.password != password {
        return (jar, Err(AuthAPIError::IncorrectCredentails))
    }

    let response = Json(LoginResponse {
        message: "User created successfully!".to_string(),
    });

    let auth_cookie =
        match generate_auth_cookie(&email) {
            Err(e) => { return (jar, Err(AuthAPIError::UnexpectedError)) },
            Ok(val) => val
        };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct LoginResponse {
    pub message: String,
}