use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {message}")]
    InternalServerError { message: String },

    #[error("GraphQL error: {0}")]
    GraphQLError(String),
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound(_) => 404,
            AppError::AuthError(_) => 401,
            AppError::ValidationError(_) => 400,
            AppError::DatabaseError(_) => 500,
            AppError::InternalServerError { .. } => 500,
            AppError::GraphQLError(_) => 400,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = Json(json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}

