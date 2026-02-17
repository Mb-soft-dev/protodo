use crate::db::TaskStore;
use colored::Colorize;
use comfy_table::Table;

pub mod protodo_commands {
    use super::*;

    pub fn add(task_store: &TaskStore, description: String) {
        match task_store.add(description.clone()) {
            Ok(task) => println!("{}", format!("Task added: {}", task.description).green()),
            Err(e) => eprintln!("Error adding task: {e}"),
        }
    }

    pub fn list(task_store: &TaskStore) {
        match task_store.list_tasks() {
            Ok(tasks) => {
                let mut table = Table::new();
                table.set_header(["ID", "Description", "Completed", "Created At"]);
                for task in tasks {
                    let max_len = 50;
                    let id_str = task.description.to_string();
                    let truncated = &id_str[..id_str.len().min(max_len)];
                    table.add_row([
                        task.id.to_string(),
                        truncated.to_string(),
                        task.completed.to_string(),
                        task.created_at.to_string(),
                    ]);
                }
                println!("{table}");
            }
            Err(e) => eprintln!("Error listing tasks: {e}"),
        }
    }

    pub fn delete(task_store: &TaskStore, id: i64) {
        match task_store.delete_task(id) {
            Ok(id) => println!("{}", format!("Task deleted: {id}").green()),
            Err(e) => eprintln!("Error delete task: {e}"),
        }
    }

    pub fn completed(task_store: &TaskStore, id: i64) {
        match task_store.complete_task(id) {
            Ok(id) => println!("{}", format!("Task completed: {id}").green()),
            Err(e) => eprintln!("Error complete task: {e}"),
        }
    }
}
