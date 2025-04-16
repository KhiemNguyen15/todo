mod config;
mod db;

use config::*;
use db::*;

use clap::{Parser, Subcommand, ValueEnum};
use prettytable::{Table, row};

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
    List {
        /// Output format: "table" (default) or "json"
        #[arg(short, long, default_value = "table")]
        format: OutputFormat,
    },
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
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

        Commands::List { format } => match get_tasks(&conn) {
            Ok(tasks) => match format {
                OutputFormat::Table => {
                    let mut table = Table::new();

                    table.add_row(row!["#", "Task", "Done"]);

                    for task in tasks {
                        table.add_row(row![
                            task.idx,
                            task.task,
                            if task.done { " ✓✓" } else { " " }
                        ]);
                    }

                    table.printstd();
                }
                OutputFormat::Json => {
                    let json =
                        serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks");

                    println!("{}", json);
                }
            },
            Err(e) => println!("Failed to list tasks: {}", e),
        },

        Commands::Done { idx } => {
            if idx == 0 {
                println!("Invalid task index");
                return;
            }

            match mark_done(&conn, idx - 1) {
                Ok(_) => println!("Marked task #{} as done", idx),
                Err(_) => println!("Task #{} not found", idx),
            }
        }

        Commands::Clean => match clear_tasks(&conn) {
            Ok(_) => println!("Cleared completed tasks"),
            Err(e) => println!("Failed to clear completed tasks: {}", e),
        },

        Commands::Reset => {
            remove_data_dir();
        }
    }
}
