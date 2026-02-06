// handlers/auth.rs
//! Authentication handlers (login, logout, refresh)

use axum::{
    extract::{Json, State, Extension},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::error::AppError;
use crate::modules::auth;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub user: UserData,
    pub session_id: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub id: String,
    pub email: String,
    pub name: String,
    pub preferences: UserPreferences,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
}

/// POST /auth/login
/// Authenticate user and return JWT token
/// POST /auth/login
/// Authenticate user and return JWT token
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    // Validate input
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::ValidationError("Email and password required".to_string()));
    }

    // TODO: Authenticate with Supabase
    // Query user from database
    let user_row: Option<(String, String, String)> = sqlx::query_as(
        "SELECT id, email, name FROM users WHERE email = $1 LIMIT 1"
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let (user_id, email, name) = user_row
        .ok_or_else(|| AppError::AuthError("Invalid credentials".to_string()))?;

    // Fetch user preferences
    let prefs: Option<(String, String, bool)> = sqlx::query_as(
        "SELECT theme, language, notifications_enabled FROM user_preferences WHERE user_id = $1"
    )
    .bind(&user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let (theme, language, notifications_enabled) = prefs.unwrap_or_else(|| {
        ("light".to_string(), "pt-BR".to_string(), true)
    });

    // Generate session ID
    let session_id = Uuid::new_v4().to_string();

    // Create session in database
    let expires_at = Utc::now() + chrono::Duration::days(30);
    sqlx::query(
        "INSERT INTO sessions (id, user_id, device_fingerprint, created_at, last_activity, expires_at, status) \
         VALUES ($1, $2, $3, NOW(), NOW(), $4, 'active')"
    )
    .bind(&session_id)
    .bind(&user_id)
    .bind("device_fp_placeholder")
    .bind(expires_at)
    .execute(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Generate access token
    let access_token = auth::generate_token(&user_id, &email)
        .map_err(|e| AppError::InternalServerError { message: format!("Failed to generate token: {}", e) })?;
    let expires_in = 3600i64;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            access_token,
            user: UserData {
                id: user_id.to_string(),
                email: email.to_string(),
                name: name.to_string(),
                preferences: UserPreferences {
                    theme,
                    language,
                    notifications_enabled,
                },
            },
            session_id,
            expires_in,
        }),
    ))
}

/// POST /auth/logout
/// Logout user and revoke session
pub async fn logout(
    State(pool): State<PgPool>,
    session_id: String,
) -> Result<StatusCode, AppError> {
    sqlx::query("UPDATE sessions SET status = 'revoked' WHERE id = $1")
        .bind(&session_id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

/// POST /auth/refresh
/// Refresh access token
pub async fn refresh_token(
    State(_pool): State<PgPool>,
    Json(_payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, AppError> {
    // TODO: Implement token refresh logic with Supabase
    Err(AppError::InternalServerError {
        message: "Token refresh not implemented".to_string(),
    })
}

/// GET /user/profile
/// Get current user profile
pub async fn get_user_profile(
    State(pool): State<PgPool>,
    user_id: String,
) -> Result<Json<UserData>, AppError> {
    let user_row: Option<(String, String, String)> = sqlx::query_as(
        "SELECT id, email, name FROM users WHERE id = $1 LIMIT 1"
    )
    .bind(&user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let (id, email, name) = user_row
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let prefs: Option<(String, String, bool)> = sqlx::query_as(
        "SELECT theme, language, notifications_enabled FROM user_preferences WHERE user_id = $1"
    )
    .bind(&user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let (theme, language, notifications_enabled) = prefs.unwrap_or_else(|| {
        ("light".to_string(), "pt-BR".to_string(), true)
    });

    Ok(Json(UserData {
        id,
        email,
        name,
        preferences: UserPreferences {
            theme,
            language,
            notifications_enabled,
        },
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub notifications_enabled: Option<bool>,
}

/// GET /user/preferences
/// Get user preferences
pub async fn get_user_preferences(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<String>,
) -> Result<Json<UserPreferences>, AppError> {
    let prefs: Option<(String, String, bool)> = sqlx::query_as(
        "SELECT theme, language, notifications_enabled FROM user_preferences WHERE user_id = $1"
    )
    .bind(&user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let (theme, language, notifications_enabled) = prefs.unwrap_or_else(|| {
        ("light".to_string(), "pt-BR".to_string(), true)
    });

    Ok(Json(UserPreferences {
        theme,
        language,
        notifications_enabled,
    }))
}

/// PATCH /user/preferences
/// Update user preferences
pub async fn update_user_preferences(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<String>,
    Json(payload): Json<UpdatePreferencesRequest>,
) -> Result<Json<UserPreferences>, AppError> {
    // Get current preferences
    let current: Option<(String, String, bool)> = sqlx::query_as(
        "SELECT theme, language, notifications_enabled FROM user_preferences WHERE user_id = $1"
    )
    .bind(&user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let (cur_theme, cur_lang, cur_notif) = current.unwrap_or_else(|| {
        ("light".to_string(), "pt-BR".to_string(), true)
    });

    let theme = payload.theme.unwrap_or(cur_theme);
    let language = payload.language.unwrap_or(cur_lang);
    let notifications_enabled = payload.notifications_enabled.unwrap_or(cur_notif);

    // Update preferences
    sqlx::query(
        "UPDATE user_preferences SET theme = $1, language = $2, notifications_enabled = $3, updated_at = NOW() WHERE user_id = $4"
    )
    .bind(&theme)
    .bind(&language)
    .bind(notifications_enabled)
    .bind(&user_id)
    .execute(&pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(Json(UserPreferences {
        theme,
        language,
        notifications_enabled,
    }))
}
