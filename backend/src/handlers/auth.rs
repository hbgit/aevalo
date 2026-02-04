// Auth Handler
// Basic authentication placeholder

use axum::{
    extract::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
}

/// POST /auth/login
/// Placeholder for authentication
pub async fn login(
    Json(req): Json<LoginRequest>,
) -> (StatusCode, Json<LoginResponse>) {
    // TODO: Implement JWT authentication
    (StatusCode::OK, Json(LoginResponse {
        token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9".to_string(),
        message: "Login successful".to_string(),
    }))
}
