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
    // Complete { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    let mut task_store = db::TaskStore::new();

    match cli.command {
        Commands::List => {
            cli::protodo_commands::list(&task_store);
        }
        Commands::Add { description } => {
            cli::protodo_commands::add(&mut task_store, description);
        } // Commands::Complete { id } => {
          //     println!("Completing task with ID: {}", id);
          //     // Here you would mark the task as complete in your storage
          // }
    }
}
