use clap::Parser;
mod cli;
mod db;
mod task;

use clap::builder::TypedValueParser;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    List {
        #[arg(short, long, default_value_t = 1)]
        page: usize,
        #[arg(long, default_value_t = 10, value_parser = clap::builder::PossibleValuesParser::new(["5", "10", "20", "30", "40", "50"]).map(|s| s.parse::<usize>().unwrap()))]
        page_size: usize,
    },
    ClearAllTasks,
    GetById {
        id: i64,
    },
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
        Commands::List { page, page_size } => {
            cli::protodo_commands::list(&task_store, Some(page), Some(page_size));
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
        Commands::ClearAllTasks => {
            cli::protodo_commands::clear_all_tasks(&task_store);
        }
        Commands::GetById { id } => {
            cli::protodo_commands::get_by_id(&task_store, id);
        }
    }
}
