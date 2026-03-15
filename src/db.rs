use std::path::PathBuf;

use chrono::Utc;
use rusqlite::Connection;

use crate::task::Task;

pub struct TaskStore {
    conn: Connection,
}

impl TaskStore {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(&db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                status INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(TaskStore { conn })
    }

    fn get_db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::data_local_dir().ok_or("Failed to get data directory")?;
        path.push("protodo");
        std::fs::create_dir_all(&path)?;
        path.push("protodo.db");
        Ok(path)
    }

    pub fn get_by_id(&self, id: i64) -> rusqlite::Result<Task> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, status, created_at FROM tasks WHERE id = ?1")?;
        let task = stmt
            .query_row([id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    status: match row.get::<_, i32>(2)? {
                        0 => crate::task::TaskStatus::Pending,
                        1 => crate::task::TaskStatus::InProgress,
                        2 => crate::task::TaskStatus::Done,
                        _ => crate::task::TaskStatus::Pending,
                    },
                    created_at: row.get(3)?,
                })
            })
            .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;
        Ok(task)
    }

    /// Clears all tasks from the database (useful for tests).
    pub fn clear_all_tasks(&self) -> rusqlite::Result<()> {
        self.conn.execute("DELETE FROM tasks", [])?;
        Ok(())
    }

    pub fn add(&self, description: String) -> rusqlite::Result<Task> {
        let now = Utc::now().naive_utc();

        self.conn.execute(
            "INSERT INTO tasks (description, created_at, status) VALUES (?1, ?2, ?3)",
            [&description as &dyn rusqlite::ToSql, &now, &(0 as i64)],
        )?;

        let id = self.conn.last_insert_rowid();
        Ok(Task {
            id,
            description,
            status: crate::task::TaskStatus::Pending,
            created_at: Utc::now(),
        })
    }

    pub fn list_tasks(&self) -> rusqlite::Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, status, created_at FROM tasks ORDER BY id ASC")?;
        let tasks_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                status: match row.get::<_, i32>(2)? {
                    0 => crate::task::TaskStatus::Pending,
                    1 => crate::task::TaskStatus::InProgress,
                    2 => crate::task::TaskStatus::Done,
                    _ => crate::task::TaskStatus::Pending,
                },
                created_at: row.get(3)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in tasks_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn delete_task(&self, id: i64) -> rusqlite::Result<i64> {
        self.conn
            .execute("DELETE FROM tasks WHERE id = (?1)", [id])?;
        Ok(id)
    }

    pub fn update_status(&self, id: i64, status: crate::task::TaskStatus) -> rusqlite::Result<i64> {
        let status_value = match status {
            crate::task::TaskStatus::Pending => 0,
            crate::task::TaskStatus::InProgress => 1,
            crate::task::TaskStatus::Done => 2,
        };
        self.conn.execute(
            "UPDATE tasks SET status = (?1) WHERE id = (?2)",
            [&status_value as &dyn rusqlite::ToSql, &id],
        )?;

        Ok(id)
    }
}
