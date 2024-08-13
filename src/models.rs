use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub is_done: bool,
    // pub created_at: String,
    // pub updated_at: String,
}

pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
}
