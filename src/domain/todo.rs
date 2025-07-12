use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoId(pub Uuid);

impl TodoId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: TodoId::new(),
            title,
            description,
            status: TodoStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: TodoStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn update_content(&mut self, title: String, description: Option<String>) {
        self.title = title;
        self.description = description;
        self.updated_at = Utc::now();
    }
}
