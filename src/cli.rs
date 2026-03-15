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
                if tasks.is_empty() {
                    println!("{}", "Protodo: no tasks found".yellow());
                    return;
                }

                let table = format_task_table(tasks);
                println!("{table}");
            }
            Err(e) => eprintln!("Error listing tasks: {e}"),
        }
    }

    /// Reusable function to format tasks into a table
    fn format_task_table(tasks: Vec<crate::task::Task>) -> Table {
        let mut table = Table::new();
        table.load_preset(comfy_table::presets::ASCII_HORIZONTAL_ONLY);
        table.set_header(["ID", "Description", "Created At", "Status"]);
        for task in tasks {
            table.add_row(format_task_row(&task));
        }
        table
    }

    /// Formats a single task into a Vec<String> row
    fn format_task_row(task: &crate::task::Task) -> Vec<String> {
        let max_len = 50;
        let truncated_description = if task.description.len() > max_len {
            format!("{}...", &task.description[..max_len])
        } else {
            task.description.clone()
        };

        vec![
            task.id.to_string(),
            truncated_description,
            task.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            match task.status {
                crate::task::TaskStatus::Pending => "Pending".blue().to_string(),
                crate::task::TaskStatus::InProgress => "InProgress".yellow().to_string(),
                crate::task::TaskStatus::Done => "Done".green().to_string(),
            },
        ]
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

    pub fn clear_all_tasks(task_store: &TaskStore) {
        match task_store.clear_all_tasks() {
            Ok(_) => println!("{}", "All tasks cleared".green()),
            Err(e) => eprintln!("Error clearing tasks: {e}"),
        }
    }
    pub fn get_by_id(task_store: &TaskStore, id: i64) {
        match task_store.get_by_id(id) {
            Ok(task) => {
                let table = format_task_table(vec![task]); // Wrap single task in a Vec
                println!("{table}");
            }
            Err(e) => eprintln!("Error getting task: {e}"),
        }
    }
}
