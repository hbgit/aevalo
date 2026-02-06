// middleware/auth.rs
//! Authentication middleware for protected routes

use axum::extract::State;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use sqlx::PgPool;

use crate::modules::auth::auth_user_from_headers;
use crate::error::AppError;

/// Middleware that requires authentication
pub async fn require_auth(
    State(pool): State<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Extract JWT and validate
    let user = auth_user_from_headers(req.headers())
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // Add user to request extensions
    req.extensions_mut().insert(user.id.to_string());
    req.extensions_mut().insert(user.email.clone());

    // Set RLS context for row-level security
    sqlx::query("SELECT set_config('request.jwt.claim.sub', $1, true)")
        .bind(user.id.to_string())
        .execute(&pool)
        .await
        .ok();

    Ok(next.run(req).await)
}

/// Middleware for optional authentication
pub async fn optional_auth(
    State(_pool): State<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    if let Ok(user) = auth_user_from_headers(req.headers()) {
        req.extensions_mut().insert(user.id.to_string());
        req.extensions_mut().insert(user.email);
    }

    next.run(req).await
}
