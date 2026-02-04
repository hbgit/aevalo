// Analytics Handler
// Processes responses and generates results

use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::error::AppError;
use crate::models::ScaleType;
use crate::modules::auth::AuthUser;

#[derive(Debug, Serialize)]
pub struct AnalyticsResult {
    pub evaluation_id: Uuid,
    pub total_responses: i32,
    pub response_rate: f64,
    pub metrics: serde_json::Value,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct QuestionMetrics {
    pub question_id: Uuid,
    pub question_text: String,
    pub statistics: Statistics,
}

#[derive(Debug, Serialize)]
pub struct Statistics {
    pub mean: Option<f64>,
    pub median: Option<f64>,
    pub std_dev: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub distribution: serde_json::Value,
    pub confidence_score: Option<f64>,
}

/// POST /evaluations/{id}/process
/// Trigger manual analytics processing
pub async fn process_evaluation(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    AuthUser { id: user_id }: AuthUser,
) -> Result<(StatusCode, Json<AnalyticsResult>), AppError> {
    // Verify ownership
    sqlx::query("SELECT id FROM evaluations WHERE id = $1 AND user_id = $2")
        .bind(eval_id)
        .bind(user_id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    // Process analytics
    let result = compute_analytics(&db, eval_id).await?;

    Ok((StatusCode::OK, Json(result)))
}

/// GET /evaluations/{id}/results
/// Retrieve processed analytics results
pub async fn get_results(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    AuthUser { id: user_id }: AuthUser,
) -> Result<(StatusCode, Json<ResultsResponse>), AppError> {
    // Verify ownership
    sqlx::query("SELECT id FROM evaluations WHERE id = $1 AND user_id = $2")
        .bind(eval_id)
        .bind(user_id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    // Get stored results or compute on-demand
    let results = sqlx::query_as::<_, (serde_json::Value,)>(
        "SELECT metrics FROM analytics_results WHERE evaluation_id = $1"
    )
    .bind(eval_id)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let metrics = if let Some((stored_metrics,)) = results {
        stored_metrics
    } else {
        // Compute on demand
        compute_analytics(&db, eval_id).await?.metrics
    };

    Ok((StatusCode::OK, Json(ResultsResponse {
        evaluation_id: eval_id,
        metrics,
    })))
}

#[derive(Debug, Serialize)]
pub struct ResultsResponse {
    pub evaluation_id: Uuid,
    pub metrics: serde_json::Value,
}

/// Compute analytics for an evaluation
async fn compute_analytics(db: &PgPool, eval_id: Uuid) -> Result<AnalyticsResult, AppError> {
    // Get evaluation details
    let eval: (String, String) = sqlx::query_as(
        "SELECT scale_type, status FROM evaluations WHERE id = $1"
    )
    .bind(eval_id)
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    let scale_type = eval.0.as_str();

    // Count total responses
    let response_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT respondent_id) FROM responses WHERE evaluation_id = $1"
    )
    .bind(eval_id)
    .fetch_one(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Get questions
    let questions: Vec<(Uuid, String)> = sqlx::query_as(
        "SELECT id, text FROM questions WHERE evaluation_id = $1 ORDER BY order ASC"
    )
    .bind(eval_id)
    .fetch_all(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Compute metrics by scale type
    let mut question_metrics = vec![];
    for (q_id, q_text) in questions {
        let metrics = compute_question_metrics(db, eval_id, q_id, &q_text, scale_type).await?;
        question_metrics.push(metrics);
    }

    let response_rate = if !question_metrics.is_empty() {
        (response_count.0 as f64 / question_metrics.len() as f64) * 100.0
    } else {
        0.0
    };

    let result = AnalyticsResult {
        evaluation_id: eval_id,
        total_responses: response_count.0 as i32,
        response_rate,
        metrics: json!({
            "by_question": question_metrics,
            "summary": {
                "total_questions": question_metrics.len(),
                "total_responses": response_count.0,
            }
        }),
        generated_at: Utc::now(),
    };

    // Store results
    sqlx::query(
        r#"
        INSERT INTO analytics_results (id, evaluation_id, total_responses, response_rate, metrics, generated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (evaluation_id) DO UPDATE SET
            total_responses = $3,
            response_rate = $4,
            metrics = $5,
            generated_at = $6
        "#
    )
    .bind(Uuid::new_v4())
    .bind(eval_id)
    .bind(response_count.0)
    .bind(response_rate)
    .bind(&result.metrics)
    .bind(result.generated_at)
    .execute(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(result)
}

/// Compute metrics for a single question
async fn compute_question_metrics(
    db: &PgPool,
    eval_id: Uuid,
    q_id: Uuid,
    q_text: &str,
    scale_type: &str,
) -> Result<QuestionMetrics, AppError> {
    // Fetch all responses for this question
    let responses: Vec<(serde_json::Value,)> = sqlx::query_as(
        "SELECT answer_value FROM responses WHERE question_id = $1 AND evaluation_id = $2"
    )
    .bind(q_id)
    .bind(eval_id)
    .fetch_all(db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let stats = match scale_type {
        "Likert" | "Frequency" => compute_numeric_statistics(&responses),
        "FixedSum" => compute_numeric_statistics(&responses),
        "PairedComparison" => compute_comparison_statistics(&responses),
        _ => Statistics {
            mean: None,
            median: None,
            std_dev: None,
            min: None,
            max: None,
            distribution: json!({}),
            confidence_score: None,
        },
    };

    Ok(QuestionMetrics {
        question_id: q_id,
        question_text: q_text.to_string(),
        statistics: stats,
    })
}

/// Compute statistics for numeric responses
fn compute_numeric_statistics(responses: &[(serde_json::Value,)]) -> Statistics {
    let values: Vec<f64> = responses
        .iter()
        .filter_map(|(v,)| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
        .collect();

    if values.is_empty() {
        return Statistics {
            mean: None,
            median: None,
            std_dev: None,
            min: None,
            max: None,
            distribution: json!({}),
            confidence_score: None,
        };
    }

    // Calculate mean
    let mean = values.iter().sum::<f64>() / values.len() as f64;

    // Calculate median
    let mut sorted = values.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    };

    // Calculate std dev
    let variance =
        values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
    let std_dev = variance.sqrt();

    // Create distribution histogram
    let mut distribution = serde_json::Map::new();
    for value in &values {
        let key = format!("{}", *value as i32);
        let count = distribution
            .get(&key)
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        distribution.insert(key, json!(count + 1));
    }

    Statistics {
        mean: Some(mean),
        median: Some(median),
        std_dev: Some(std_dev),
        min: values.iter().cloned().fold(f64::INFINITY, f64::min).into(),
        max: values.iter().cloned().fold(f64::NEG_INFINITY, f64::max).into(),
        distribution: json!(distribution),
        confidence_score: Some(0.95), // Placeholder
    }
}

/// Compute statistics for paired comparison responses
fn compute_comparison_statistics(responses: &[(serde_json::Value,)]) -> Statistics {
    let mut wins = serde_json::Map::new();

    for response in responses {
        if let Some(obj) = response.0.as_object() {
            for (item, win) in obj {
                if win.as_bool().unwrap_or(false) {
                    let count = wins
                        .get(item)
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    wins.insert(item.clone(), json!(count + 1));
                }
            }
        }
    }

    Statistics {
        mean: None,
        median: None,
        std_dev: None,
        min: None,
        max: None,
        distribution: json!(wins),
        confidence_score: Some(0.85),
    }
}
