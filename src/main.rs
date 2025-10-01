use clap::Parser;
mod cli;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    List,
    Add { description: String },
    Complete { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    let tasks = cli::list();

    match cli.command {
        Commands::List => tasks,
        Commands::Add { description } => {
            println!("Adding task: {}", description);
            // Here you would add the task to your storage
        }
        Commands::Complete { id } => {
            println!("Completing task with ID: {}", id);
            // Here you would mark the task as complete in your storage
        }
    }
}
