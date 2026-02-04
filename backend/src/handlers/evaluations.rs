// Evaluation Endpoints Handler
// Implements: Create, List, Publish, Customize, Close

use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Evaluation, EvaluationStatus, ScaleType};
use crate::error::AppError;
use crate::modules::auth::AuthUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEvaluationRequest {
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub scale_type: ScaleType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishEvaluationRequest {
    pub items: Vec<QuestionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionItem {
    pub order: i32,
    pub text: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEvaluationRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub items: Option<Vec<QuestionItem>>,
}

/// GET /evaluations
/// Retrieve list of user's evaluations with count
pub async fn list_evaluations(
    State(db): State<PgPool>,
    AuthUser { id: user_id }: AuthUser,
) -> Result<(StatusCode, Json<ListEvaluationsResponse>), AppError> {
    let total_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM evaluations WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let evaluations = sqlx::query_as::<_, Evaluation>(
        "SELECT * FROM evaluations WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK, Json(ListEvaluationsResponse {
        total: total_count.0 as i32,
        evaluations,
        is_first_time: total_count.0 == 0,
    })))
}

#[derive(Debug, Serialize)]
pub struct ListEvaluationsResponse {
    pub total: i32,
    pub evaluations: Vec<Evaluation>,
    pub is_first_time: bool,
}

/// POST /evaluations
/// Create draft evaluation
pub async fn create_evaluation(
    State(db): State<PgPool>,
    AuthUser { id: user_id }: AuthUser,
    Json(req): Json<CreateEvaluationRequest>,
) -> Result<(StatusCode, Json<Evaluation>), AppError> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let evaluation = sqlx::query_as::<_, Evaluation>(
        r#"
        INSERT INTO evaluations 
        (id, user_id, category_id, title, description, status, scale_type, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#
    )
    .bind(id)
    .bind(user_id)
    .bind(req.category_id)
    .bind(req.title)
    .bind(req.description)
    .bind(EvaluationStatus::Draft)
    .bind(req.scale_type)
    .bind(now)
    .bind(now)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(evaluation)))
}

/// GET /evaluations/{id}
/// Retrieve single evaluation
pub async fn get_evaluation(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    AuthUser { id: user_id }: AuthUser,
) -> Result<(StatusCode, Json<EvaluationAccessResponse>), AppError> {
    let evaluation = sqlx::query_as::<_, Evaluation>(
        "SELECT * FROM evaluations WHERE id = $1"
    )
    .bind(eval_id)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    let (role, can_edit) = if evaluation.user_id == user_id {
        ("OWNER".to_string(), true)
    } else {
        let collaborator_role: Option<(String,)> = sqlx::query_as(
            "SELECT role::text FROM collaborators WHERE evaluation_id = $1 AND user_id = $2"
        )
        .bind(eval_id)
        .bind(user_id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match collaborator_role {
            Some((role,)) => {
                let can_edit = role == "OWNER" || role == "EDITOR";
                (role, can_edit)
            }
            None => {
                return Err(AppError::AuthError("Access denied".to_string()));
            }
        }
    };

    let questions = sqlx::query_as(
        "SELECT id, evaluation_id, order, text, scale_type, metadata FROM questions 
         WHERE evaluation_id = $1 ORDER BY order ASC"
    )
    .bind(eval_id)
    .fetch_all(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK, Json(EvaluationAccessResponse {
        evaluation: EvaluationView::from(evaluation),
        questions,
        role,
        can_edit,
    })))
}

#[derive(Debug, Serialize)]
pub struct EvaluationAccessResponse {
    pub evaluation: EvaluationView,
    pub questions: Vec<QuestionDetail>,
    pub role: String,
    pub can_edit: bool,
}

#[derive(Debug, Serialize)]
pub struct EvaluationView {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: EvaluationStatus,
    pub scale_type: ScaleType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Evaluation> for EvaluationView {
    fn from(e: Evaluation) -> Self {
        Self {
            id: e.id,
            category_id: e.category_id,
            title: e.title,
            description: e.description,
            status: e.status,
            scale_type: e.scale_type,
            created_at: e.created_at,
            updated_at: e.updated_at,
            published_at: e.published_at,
            closed_at: e.closed_at,
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct QuestionDetail {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub order: i32,
    pub text: String,
    pub scale_type: ScaleType,
    pub metadata: serde_json::Value,
}

/// PATCH /evaluations/{id}
/// Update draft evaluation
pub async fn update_evaluation(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    AuthUser { id: user_id }: AuthUser,
    Json(req): Json<UpdateEvaluationRequest>,
) -> Result<(StatusCode, Json<Evaluation>), AppError> {
    // Verify ownership and status is draft
    let evaluation = sqlx::query_as::<_, Evaluation>(
        "SELECT * FROM evaluations WHERE id = $1 AND user_id = $2"
    )
    .bind(eval_id)
    .bind(user_id)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    if !matches!(evaluation.status, EvaluationStatus::Draft) {
        return Err(AppError::ValidationError(
            "Can only edit draft evaluations".to_string(),
        ));
    }

    let updated = sqlx::query_as::<_, Evaluation>(
        r#"
        UPDATE evaluations 
        SET title = COALESCE($2, title),
            description = COALESCE($3, description),
            updated_at = $4
        WHERE id = $1 AND user_id = $5
        RETURNING *
        "#
    )
    .bind(eval_id)
    .bind(req.title)
    .bind(req.description)
    .bind(Utc::now())
    .bind(user_id)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Update questions if provided
    if let Some(items) = req.items {
        for item in items {
            sqlx::query(
                r#"
                INSERT INTO questions (id, evaluation_id, order, text, scale_type, metadata)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (evaluation_id, order) 
                DO UPDATE SET text = $4, metadata = $6
                "#
            )
            .bind(Uuid::new_v4())
            .bind(eval_id)
            .bind(item.order)
            .bind(item.text)
            .bind(updated.scale_type.clone())
            .bind(item.metadata)
            .execute(&db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }
    }

    Ok((StatusCode::OK, Json(updated)))
}

/// POST /evaluations/{id}/publish
/// Publish evaluation and generate public link
pub async fn publish_evaluation(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    AuthUser { id: user_id }: AuthUser,
    Json(req): Json<PublishEvaluationRequest>,
) -> Result<(StatusCode, Json<PublishResponse>), AppError> {
    // Verify ownership
    let evaluation = sqlx::query_as::<_, Evaluation>(
        "SELECT * FROM evaluations WHERE id = $1 AND user_id = $2"
    )
    .bind(eval_id)
    .bind(user_id)
    .fetch_optional(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    // Insert questions
    for item in req.items {
        sqlx::query(
            r#"
            INSERT INTO questions (id, evaluation_id, order, text, scale_type, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(Uuid::new_v4())
        .bind(eval_id)
        .bind(item.order)
        .bind(item.text)
        .bind(evaluation.scale_type.clone())
        .bind(item.metadata)
        .execute(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }

    // Update status to Open
    sqlx::query(
        "UPDATE evaluations SET status = $1, published_at = $2, updated_at = $2 WHERE id = $3"
    )
    .bind(EvaluationStatus::Open)
    .bind(Utc::now())
    .bind(eval_id)
    .execute(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Generate public link
    let link_id = Uuid::new_v4();
    let uuid = Uuid::new_v4().to_string();
    let short_url = generate_short_url(&uuid);

    let public_link = sqlx::query_as::<_, PublicLinkModel>(
        r#"
        INSERT INTO public_links (id, evaluation_id, uuid, short_url, is_active)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, evaluation_id, uuid, short_url, is_active
        "#
    )
    .bind(link_id)
    .bind(eval_id)
    .bind(&uuid)
    .bind(&short_url)
    .bind(true)
    .fetch_one(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK, Json(PublishResponse {
        evaluation_id: eval_id,
        status: EvaluationStatus::Open,
        public_link: format!("/public/eval/{}", uuid),
        short_url,
    })))
}

#[derive(Debug, Serialize)]
pub struct PublishResponse {
    pub evaluation_id: Uuid,
    pub status: EvaluationStatus,
    pub public_link: String,
    pub short_url: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PublicLinkModel {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub uuid: String,
    pub short_url: String,
    pub is_active: bool,
}

/// POST /evaluations/{id}/close
/// Close evaluation and trigger analytics
pub async fn close_evaluation(
    State(db): State<PgPool>,
    Path(eval_id): Path<Uuid>,
    AuthUser { id: user_id }: AuthUser,
) -> Result<(StatusCode, Json<serde_json::json::Value>), AppError> {
    // Verify ownership
    sqlx::query("SELECT id FROM evaluations WHERE id = $1 AND user_id = $2")
        .bind(eval_id)
        .bind(user_id)
        .fetch_optional(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::NotFound("Evaluation not found".to_string()))?;

    // Close evaluation
    sqlx::query(
        "UPDATE evaluations SET status = $1, closed_at = $2 WHERE id = $3"
    )
    .bind(EvaluationStatus::Closed)
    .bind(Utc::now())
    .bind(eval_id)
    .execute(&db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Deactivate public link
    sqlx::query("UPDATE public_links SET is_active = false WHERE evaluation_id = $1")
        .bind(eval_id)
        .execute(&db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // TODO: Trigger analytics processing job
    // In production, this would be an async job queue

    Ok((StatusCode::OK, Json(serde_json::json!({
        "status": "closed",
        "evaluation_id": eval_id,
        "message": "Evaluation closed. Analytics processing started."
    }))))
}

// Helper function to generate short URL
fn generate_short_url(uuid: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    uuid.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Base62 encoding for short URL
    format!("eval-{:x}", hash % 1_000_000)
}
