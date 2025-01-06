use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, serve::Serve, Router};

pub async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}