use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, serve::Serve, Router};

pub async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}