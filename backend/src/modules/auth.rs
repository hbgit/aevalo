//! Authentication module for user validation and JWT handling

use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderMap},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    /// Checks if token is expired
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.exp
    }

    /// Gets time remaining until expiry in seconds
    pub fn time_remaining(&self) -> i64 {
        (self.exp - chrono::Utc::now().timestamp()).max(0)
    }
}

/// Authenticated user context
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Some(user) = parts.extensions.get::<AuthUser>() {
            return Ok(user.clone());
        }

        auth_user_from_headers(&parts.headers)
    }
}

/// Extracts authenticated user from request headers
pub fn auth_user_from_headers(headers: &HeaderMap) -> Result<AuthUser, AppError> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::AuthError("Missing Authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::AuthError("Invalid Authorization header".to_string()))?;

    let claims = verify_token(token).map_err(AppError::AuthError)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::AuthError("Invalid token subject".to_string()))?;

    Ok(AuthUser { 
        id: user_id,
        email: claims.email,
    })
}

pub fn generate_token(user_id: &str, email: &str) -> Result<String, String> {
    generate_token_with_ttl(user_id, email, Duration::hours(24))
}

pub fn generate_refresh_token(user_id: &str, email: &str) -> Result<String, String> {
    generate_token_with_ttl(user_id, email, Duration::days(7))
}

fn generate_token_with_ttl(user_id: &str, email: &str, ttl: Duration) -> Result<String, String> {
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + ttl).timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat,
        exp,
    };

    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key".to_string());
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Failed to encode token: {}", e))
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key".to_string());
    
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Failed to verify token: {}", e))
}
