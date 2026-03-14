use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Done,
}
