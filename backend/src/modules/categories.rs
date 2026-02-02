// Categories module
// Organizational structure for evaluations

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

impl Category {
    pub fn new(user_id: Uuid, name: String, color: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            description: None,
            color,
            created_at: Utc::now(),
        }
    }
}
