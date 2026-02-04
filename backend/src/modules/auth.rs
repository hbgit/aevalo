// Authentication module
// Handles JWT token generation and validation

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub iat: i64,
    pub exp: i64,
}

pub fn generate_token(user_id: &str) -> Result<String, String> {
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(24)).timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
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
