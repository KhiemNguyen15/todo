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
        id: i32,
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
        Commands::Add { task } => {
            add_task(&conn, &task).expect("Failed to add task");
        }
        Commands::List => {
            list_tasks(&conn).expect("Failed to list tasks");
        }
        Commands::Done { id } => {
            mark_done(&conn, id).expect("Failed to mark task as done");
        }
        Commands::Clean => {
            clear_tasks(&conn).expect("Failed to clear completed tasks");
        }
        Commands::Reset => {
            remove_data_dir();
        }
    }
}
