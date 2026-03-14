use clap::Parser;
mod cli;
mod db;
mod task;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    List,
    Add {
        description: String,
    },
    Delete {
        id: i64,
    },
    UpdateStatus {
        id: i64,
        status: crate::task::TaskStatus,
    },
}

fn main() {
    let cli = Cli::parse();
    let task_store = match db::TaskStore::new() {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Error initializing database: {e}");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::List => {
            cli::protodo_commands::list(&task_store);
        }
        Commands::Add { description } => {
            cli::protodo_commands::add(&task_store, description);
        }
        Commands::Delete { id } => {
            cli::protodo_commands::delete(&task_store, id);
        }
        Commands::UpdateStatus { id, status } => {
            cli::protodo_commands::update_status(&task_store, id, status);
        }
    }
}
