use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password}};

pub async fn login(State(state): State<AppState>,
Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let user_store = state.user_store.write().await;
    
    let email = Email::parse(&request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = user_store.get_user(&email).await.map_err(|_| AuthAPIError::InvalidCredentials)?;

    if user.password != password {
        return Err(AuthAPIError::IncorrectCredentails)
    }

    let response = Json(LoginResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
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