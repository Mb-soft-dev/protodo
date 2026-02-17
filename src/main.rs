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
    Add { description: String },
    Delete { id: i64 },
    Completed { id: i64 },
}

fn main() {
    let cli = Cli::parse();
    let task_store = db::TaskStore::new();

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

        Commands::Completed { id } => {
            cli::protodo_commands::completed(&task_store, id);
        }
    }
}
