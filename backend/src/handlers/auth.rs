// Auth Handler
// Basic authentication placeholder

use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::modules::auth::generate_token;

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
/// Authenticate user and return JWT token
pub async fn login(
    State(db): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    let user: (Uuid, String) = sqlx::query_as(
        "SELECT id, password_hash FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::AuthError("Invalid credentials".to_string()))?;

    let password_matches = verify(&req.password, &user.1)
        .map_err(|_| AppError::AuthError("Invalid credentials".to_string()))?;

    if !password_matches {
        return Err(AppError::AuthError("Invalid credentials".to_string()));
    }

    let token = generate_token(&user.0.to_string())
        .map_err(|e| AppError::InternalServerError { message: e })?;

    Ok((StatusCode::OK, Json(LoginResponse {
        token,
        message: "Login successful".to_string(),
    })))
}
