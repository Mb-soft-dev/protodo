use colored::*;
use comfy_table::{Cell, Table};
pub struct Task {
    pub description: String,
}
pub fn list() -> () {
    let mut table = Table::new();
    table.set_header(vec!["ID", "Description", "Status"]);
    table.add_row(vec![
        Cell::new("1"),
        Cell::new("Learn Python"),
        Cell::new("In Progress").fg(comfy_table::Color::Yellow),
    ]);

    table.add_row(vec![
        Cell::new("2"),
        Cell::new("Learn Rust"),
        Cell::new("Not Started").fg(comfy_table::Color::Red),
    ]);

    println!("{table}");

    // vec![
    //     Task{description: "learn python".to_string()},
    //     Task{description: "learn rust".to_string()},
    // ]
}
