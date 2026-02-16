pub mod protodo_commands {
    use crate::db::TaskStore;
    use comfy_table::{Cell, Color, Table, presets};

    pub fn add(store: &mut TaskStore, description: String) {
        let id = store.list().len() as u32 + 1;
        TaskStore::add(store, description);
        println!("Added task: {id}");
    }

    pub fn list(store: &TaskStore) {
        let mut table = Table::new();
        table.set_header(vec!["ID", "Description", "Status"]);

        if store.list().is_empty() {
            println!("No tasks found.");
            return;
        }

        let rows: Vec<Vec<Cell>> = store
            .list()
            .iter()
            .map(|task| {
                let status = if task.completed {
                    "finished"
                } else {
                    "pending"
                };
                vec![
                    Cell::new(task.id.to_string()).fg(Color::Green),
                    Cell::new(task.description.clone()).fg(Color::Yellow),
                    Cell::new(status).fg(Color::Blue),
                ]
            })
            .collect();

        table.add_rows(rows);

        table.load_preset(presets::UTF8_FULL);

        println!("{table}");
    }
}
