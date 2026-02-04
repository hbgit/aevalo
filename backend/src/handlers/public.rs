// Public Endpoints Handler
// Handles unauthenticated access to evaluations via public links

use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct PublicEvaluationResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub scale_type: String,
    pub questions: Vec<PublicQuestion>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicQuestion {
    pub id: Uuid,
    pub order: i32,
    pub text: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct PublicStatsResponse {
    pub evaluation_id: Uuid,
    pub total_responses: i32,
    pub response_rate: f64,
}

/// GET /public/eval/{uuid}
/// Retrieve evaluation for public response
pub async fn get_public_evaluation(
    State(db): State<PgPool>,
    Path(uuid): Path<String>,
) -> Result<(StatusCode, Json<PublicEvaluationResponse>), AppError> {
    // Verify public link exists and is active
    let eval_id: (Uuid,) = sqlx::query_as(
        "SELECT evaluation_id FROM public_links WHERE uuid = $1 AND is_active = true"
    )
    .bind(&uuid)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation link not found or expired".to_string()))?;

    // Fetch evaluation
    let eval_row: (String, Option<String>, String) = sqlx::query_as(
        "SELECT title, description, scale_type FROM evaluations WHERE id = $1 AND status = $2"
    )
    .bind(eval_id.0)
    .bind("Open")
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation not found or closed".to_string()))?;

    // Fetch questions
    let questions = sqlx::query_as::<_, PublicQuestion>(
        "SELECT id, order, text, metadata FROM questions WHERE evaluation_id = $1 ORDER BY order ASC"
    )
    .bind(eval_id.0)
    .fetch_all(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK, Json(PublicEvaluationResponse {
        id: eval_id.0,
        title: eval_row.0,
        description: eval_row.1,
        scale_type: eval_row.2,
        questions,
    })))
}

/// GET /public/eval/{uuid}/stats
/// Retrieve response statistics (real-time dashboard)
pub async fn get_public_stats(
    State(db): State<PgPool>,
    Path(uuid): Path<String>,
) -> Result<(StatusCode, Json<PublicStatsResponse>), AppError> {
    // Verify public link
    let eval_id: (Uuid,) = sqlx::query_as(
        "SELECT evaluation_id FROM public_links WHERE uuid = $1 AND is_active = true"
    )
    .bind(&uuid)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation link not found".to_string()))?;

    // Count total responses
    let response_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT respondent_id) FROM responses WHERE evaluation_id = $1"
    )
    .bind(eval_id.0)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Get question count for response rate
    let question_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM questions WHERE evaluation_id = $1"
    )
    .bind(eval_id.0)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let response_rate = if question_count.0 > 0 {
        (response_count.0 as f64 / question_count.0 as f64) * 100.0
    } else {
        0.0
    };

    Ok((StatusCode::OK, Json(PublicStatsResponse {
        evaluation_id: eval_id.0,
        total_responses: response_count.0 as i32,
        response_rate,
    })))
}
