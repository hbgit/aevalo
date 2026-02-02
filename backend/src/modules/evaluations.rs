// Evaluations module
// Core business logic for evaluation management

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationStatus {
    Draft,
    Open,
    Closed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScaleType {
    Likert,
    Frequency,
    PairedComparison,
    FixedSum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evaluation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: EvaluationStatus,
    pub scale_type: ScaleType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
}

impl Evaluation {
    pub fn new(
        user_id: Uuid,
        category_id: Uuid,
        title: String,
        scale_type: ScaleType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            category_id,
            title,
            description: None,
            status: EvaluationStatus::Draft,
            scale_type,
            created_at: now,
            updated_at: now,
            published_at: None,
            closed_at: None,
        }
    }

    pub fn publish(&mut self) {
        self.status = EvaluationStatus::Open;
        self.published_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn close(&mut self) {
        self.status = EvaluationStatus::Closed;
        self.closed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}
