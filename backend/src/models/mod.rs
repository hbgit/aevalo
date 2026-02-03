// Type definitions and models for database entities

use async_graphql::{SimpleObject, Enum, InputObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== ENUMERATIONS ====================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Enum, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "evaluation_status", rename_all = "UPPERCASE")]
pub enum EvaluationStatus {
    Draft,
    Open,
    Closed,
    Archived,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Enum, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "scale_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScaleType {
    Likert,
    Frequency,
    PairedComparison,
    FixedSum,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Enum, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "collaborator_role", rename_all = "UPPERCASE")]
pub enum CollaboratorRole {
    Owner,
    Editor,
    Viewer,
}

// ==================== CORE ENTITIES ====================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct Evaluation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: EvaluationStatus,
    pub scale_type: ScaleType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct Question {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub order: i32,
    pub text: String,
    pub scale_type: ScaleType,
    pub metadata: sqlx::types::JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct Response {
    pub id: Uuid,
    pub question_id: Uuid,
    pub evaluation_id: Uuid,
    pub respondent_id: String,
    pub answer_value: sqlx::types::JsonValue,
    pub created_at: DateTime<Utc>,
}

// ==================== COLLABORATION & ACCESS ====================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct PublicLink {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub uuid: String,
    pub short_url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct Collaborator {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub user_id: Uuid,
    pub role: CollaboratorRole,
    pub added_at: DateTime<Utc>,
}

// ==================== TEMPLATES ====================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scale_type: ScaleType,
    pub structure: sqlx::types::JsonValue,
    pub created_at: DateTime<Utc>,
}

// ==================== ANALYTICS ====================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct AnalyticsResult {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub total_responses: i32,
    pub response_rate: f64,
    pub generated_at: DateTime<Utc>,
    pub metrics: sqlx::types::JsonValue,
    pub insights: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Statistics {
    pub mean: Option<f64>,
    pub median: Option<f64>,
    pub std_dev: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub distribution: sqlx::types::JsonValue,
    pub confidence_score: Option<f64>,
}

// ==================== SCALE TYPES ====================

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct LikertScale {
    pub min_value: i32,
    pub max_value: i32,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct FrequencyScale {
    pub categories: Vec<String>,
    pub frequency_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PairedComparisonScale {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct FixedSumScale {
    pub total_sum: i32,
    pub items: Vec<String>,
}

// ==================== INPUT TYPES ====================

#[derive(Debug, InputObject)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, InputObject)]
pub struct CreateCategoryInput {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}

#[derive(Debug, InputObject)]
pub struct CreateEvaluationInput {
    pub category_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub scale_type: ScaleType,
}

#[derive(Debug, InputObject)]
pub struct CreateQuestionInput {
    pub evaluation_id: Uuid,
    pub text: String,
    pub scale_type: ScaleType,
    pub metadata: String, // JSON as string
}

#[derive(Debug, InputObject)]
pub struct CreateResponseInput {
    pub question_id: Uuid,
    pub evaluation_id: Uuid,
    pub respondent_id: String,
    pub answer_value: String, // JSON as string
}

#[derive(Debug, InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
