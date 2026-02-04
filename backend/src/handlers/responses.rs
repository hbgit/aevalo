// Responses Handler
// Handles submission and retrieval of evaluation responses

use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct SubmitResponseRequest {
    pub respondent_id: String, // Hashed IP or anonymous ID
    pub answers: Vec<AnswerInput>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnswerInput {
    pub question_id: Uuid,
    pub answer_value: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SubmitResponseResponse {
    pub message: String,
    pub response_count: i32,
}

/// POST /responses
/// Submit responses for an evaluation
pub async fn submit_responses(
    State(db): State<PgPool>,
    Json(req): Json<SubmitResponseRequest>,
) -> Result<(StatusCode, Json<SubmitResponseResponse>), AppError> {
    // Validate answers format
    if req.answers.is_empty() {
        return Err(AppError::ValidationError(
            "At least one answer required".to_string(),
        ));
    }

    // Get evaluation_id from first question
    let eval_id: (Uuid,) = sqlx::query_as(
        "SELECT evaluation_id FROM questions WHERE id = $1"
    )
    .bind(req.answers[0].question_id)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Question not found".to_string()))?;

    // Verify evaluation is open
    let is_open: (bool,) = sqlx::query_as(
        "SELECT status = $1 FROM evaluations WHERE id = $2"
    )
    .bind("Open")
    .bind(eval_id.0)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    if !is_open.0 {
        return Err(AppError::ValidationError(
            "Evaluation is not open for responses".to_string(),
        ));
    }

    // Validate answers by scale type
    validate_answers(&db, eval_id.0, &req.answers).await?;

    // Insert responses (anonymized)
    for answer in req.answers {
        sqlx::query(
            r#"
            INSERT INTO responses (id, question_id, evaluation_id, respondent_id, answer_value, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(answer.question_id)
        .bind(eval_id.0)
        .bind(&req.respondent_id)
        .bind(answer.answer_value)
        .bind(Utc::now())
        .execute(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }

    // Count total unique respondents
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT respondent_id) FROM responses WHERE evaluation_id = $1"
    )
    .bind(eval_id.0)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(SubmitResponseResponse {
        message: "✓ Obrigado por responder".to_string(),
        response_count: count.0 as i32,
    })))
}

/// GET /evaluations/{id}/responses
/// Get all responses for an evaluation (owner only)
pub async fn get_responses(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    user_id: Uuid,
) -> Result<(StatusCode, Json<Vec<ResponseDetail>>), AppError> {
    // Verify ownership
    sqlx::query("SELECT id FROM evaluations WHERE id = $1 AND user_id = $2")
        .bind(eval_id)
        .bind(user_id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    let responses = sqlx::query_as::<_, ResponseDetail>(
        r#"
        SELECT r.id, r.question_id, r.evaluation_id, r.respondent_id, r.answer_value, r.created_at
        FROM responses r
        WHERE r.evaluation_id = $1
        ORDER BY r.created_at DESC
        "#
    )
    .bind(eval_id)
    .fetch_all(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK, Json(responses)))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ResponseDetail {
    pub id: Uuid,
    pub question_id: Uuid,
    pub evaluation_id: Uuid,
    pub respondent_id: String,
    pub answer_value: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Validate answers based on scale type
async fn validate_answers(
    db: &PgPool,
    eval_id: Uuid,
    answers: &[AnswerInput],
) -> Result<(), AppError> {
    // Get evaluation scale type
    let scale_type: (String,) = sqlx::query_as(
        "SELECT scale_type FROM evaluations WHERE id = $1"
    )
    .bind(eval_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    match scale_type.0.as_str() {
        "Likert" => {
            // Validate 1-5 range
            for answer in answers {
                if let Some(value) = answer.answer_value.as_i64() {
                    if value < 1 || value > 5 {
                        return Err(AppError::ValidationError(
                            "Likert scale must be 1-5".to_string(),
                        ));
                    }
                }
            }
        }
        "FixedSum" => {
            // Validate total sum = 100
            let total: i64 = answers
                .iter()
                .filter_map(|a| a.answer_value.as_i64())
                .sum();

            if total != 100 {
                return Err(AppError::ValidationError(
                    format!("Fixed sum must equal 100, got {}", total),
                ));
            }
        }
        _ => {} // Other types don't require special validation
    }

    Ok(())
}

/// GET /evaluations/{id}/stats
/// Get response statistics and progress
pub async fn get_response_stats(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    user_id: Uuid,
) -> Result<(StatusCode, Json<ResponseStats>), AppError> {
    // Verify ownership
    sqlx::query("SELECT id FROM evaluations WHERE id = $1 AND user_id = $2")
        .bind(eval_id)
        .bind(user_id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    // Count responses
    let response_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT respondent_id) FROM responses WHERE evaluation_id = $1"
    )
    .bind(eval_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Count questions
    let question_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM questions WHERE evaluation_id = $1"
    )
    .bind(eval_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK, Json(ResponseStats {
        total_responses: response_count.0 as i32,
        total_questions: question_count.0 as i32,
        response_rate: if question_count.0 > 0 {
            (response_count.0 as f64 / question_count.0 as f64) * 100.0
        } else {
            0.0
        },
    })))
}

#[derive(Debug, Serialize)]
pub struct ResponseStats {
    pub total_responses: i32,
    pub total_questions: i32,
    pub response_rate: f64,
}
