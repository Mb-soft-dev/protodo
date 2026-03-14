use crate::db::TaskStore;
use colored::Colorize;
use comfy_table::Table;

pub mod protodo_commands {
    use super::*;

    pub fn add(task_store: &TaskStore, description: String) {
        match task_store.add(description.clone().trim().to_string()) {
            Ok(task) => println!("{}", format!("Task added: {}", task.description).green()),
            Err(e) => eprintln!("Error adding task: {e}"),
        }
    }

    pub fn list(task_store: &TaskStore) {
        match task_store.list_tasks() {
            Ok(tasks) => {
                let mut table = Table::new();
                table.load_preset(comfy_table::presets::ASCII_HORIZONTAL_ONLY);

                table.set_header(["ID", "Description", "created_at", "Status"]);
                let max_len = 50;
                for task in tasks {
                    let truncated = if task.description.len() > max_len {
                        format!("{}...", &task.description[..max_len]).to_string()
                    } else {
                        task.description.clone().to_string()
                    };
                    table.add_row([
                        task.id.to_string(),
                        truncated,
                        task.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                        match task.status {
                            crate::task::TaskStatus::Pending => "Pending".to_string(),
                            crate::task::TaskStatus::InProgress => "InProgress".red().to_string(),
                            crate::task::TaskStatus::Done => "Done".green().to_string(),
                        },
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

    pub fn update_status(task_store: &TaskStore, id: i64, status: crate::task::TaskStatus) {
        match task_store.update_status(id, status) {
            Ok(id) => println!("{}", format!("Task completed: {id}").green()),
            Err(e) => eprintln!("Error complete task: {e}"),
        }
    }
}
