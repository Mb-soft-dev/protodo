use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: i64,
    pub description: String,
    // pub status: TaskStatus,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum TaskStatus {
//     Pending,
//     // InProgress,
//     // Done,
// }
