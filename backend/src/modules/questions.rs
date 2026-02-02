// Questions module
// Handles individual questions within evaluations

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub order: i32,
    pub text: String,
    pub metadata: Value, // Stores scale-specific configuration
}

impl Question {
    pub fn new(evaluation_id: Uuid, order: i32, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            evaluation_id,
            order,
            text,
            metadata: serde_json::json!({}),
        }
    }
}
