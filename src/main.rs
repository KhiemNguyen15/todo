mod config;
mod db;

use config::*;
use db::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        #[arg(help = "The task description")]
        task: String,
    },
    /// List all todo items
    List,
    /// Mark a task as done
    Done {
        #[arg(help = "The task ID to mark as done")]
        idx: usize,
    },
    /// Remove all tasks marked as done
    Clean,
    /// Remove the data directory
    Reset,
}

fn main() {
    let cli = Cli::parse();
    let db_path = get_data_path();
    let conn = init_db(&db_path).expect("Failed to initialize database");

    match cli.command {
        Commands::Add { task } => match add_task(&conn, &task) {
            Ok(_) => println!("Added task: {}", task),
            Err(e) => println!("Failed to add task: {}", e),
        },

        Commands::List => match list_tasks(&conn) {
            Ok(_) => (),
            Err(e) => println!("Failed to list tasks: {}", e),
        },

        Commands::Done { idx } => match mark_done(&conn, idx) {
            Ok(_) => println!("Marked task #{} as done", idx),
            Err(e) => println!("Failed to mark task as done: {}", e),
        },

        Commands::Clean => match clear_tasks(&conn) {
            Ok(_) => println!("Cleared completed tasks"),
            Err(e) => println!("Failed to clear completed tasks: {}", e),
        },

        Commands::Reset => {
            remove_data_dir();
        }
    }
}
