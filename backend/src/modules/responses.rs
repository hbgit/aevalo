// Responses module
// Handles collected responses from evaluators

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: Uuid,
    pub question_id: Uuid,
    pub evaluation_id: Uuid,
    pub respondent_id: String, // Anonymous or UUID
    pub answer_value: Value,
    pub created_at: DateTime<Utc>,
}

impl Response {
    pub fn new(
        question_id: Uuid,
        evaluation_id: Uuid,
        respondent_id: String,
        answer_value: Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            question_id,
            evaluation_id,
            respondent_id,
            answer_value,
            created_at: Utc::now(),
        }
    }
}
