use chrono::Utc;
use rusqlite::Connection;

use crate::task::Task;

pub struct TaskStore {
    pub conn: Connection,
}

impl TaskStore {
    pub fn new() -> Self {
        let conn = Connection::open("protodo.db").expect("Failed to connect db");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create table");

        TaskStore { conn }
    }

    pub fn add(&self, description: String) -> rusqlite::Result<Task> {
        let now = Utc::now().naive_utc();

        self.conn.execute(
            "INSERT INTO tasks (description, created_at) VALUES (?1, ?2)",
            [&description as &dyn rusqlite::ToSql, &now],
        )?;

        let id = self.conn.last_insert_rowid();
        Ok(Task {
            id,
            description,
            completed: false,
            created_at: Utc::now(),
        })
    }

    pub fn list_tasks(&self) -> rusqlite::Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, completed, created_at FROM tasks ORDER BY id")?;
        let tasks_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
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

    pub fn complete_task(&self, id: i64) -> rusqlite::Result<i64> {
        self.conn
            .execute("UPDATE tasks SET completed = 1 WHERE id = (?1)", [id])?;

        Ok(id)
    }
}
