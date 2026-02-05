use axum::extract::State;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use sqlx::PgPool;

use crate::error::AppError;
use crate::modules::auth::auth_user_from_headers;

pub async fn require_auth(
    State(pool): State<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let user = auth_user_from_headers(req.headers())?;

    sqlx::query("SELECT set_config('request.jwt.claim.sub', $1, true)")
        .bind(user.id.to_string())
        .execute(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
