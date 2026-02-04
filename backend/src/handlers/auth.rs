// Auth Handler
// Basic authentication placeholder

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::modules::auth::{generate_refresh_token, generate_token};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub message: String,
}

/// POST /auth/login
/// Authenticate user and return JWT token
pub async fn login(
    State(db): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    let (user_id, _email, _name): (Uuid, String, String) = sqlx::query_as(
        "SELECT id, email, name FROM auth_login($1, $2)"
    )
    .bind(&req.email)
    .bind(&req.password)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::AuthError("Invalid credentials".to_string()))?;

    let token = generate_token(&user_id.to_string())
        .map_err(|e| AppError::InternalServerError { message: e })?;

    let refresh_token = generate_refresh_token(&user_id.to_string())
        .map_err(|e| AppError::InternalServerError { message: e })?;

    Ok((StatusCode::OK, Json(LoginResponse {
        token,
        refresh_token,
        message: "Login successful".to_string(),
    })))
}
