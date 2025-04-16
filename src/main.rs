mod config;
mod db;

use chrono::NaiveDate;
use config::*;
use db::*;

use clap::{Parser, Subcommand, ValueEnum};
use prettytable::{Table, row};

#[derive(Parser)]
#[command(name = "todo", version, about = "A simple CLI todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        #[arg(help = "The task description")]
        task: String,

        #[arg(long, help = "Due date (YYYY-MM-DD")]
        due: Option<String>,
    },
    /// List all tasks
    List {
        /// Output format: "table" (default) or "json"
        #[arg(short, long, default_value = "table")]
        format: OutputFormat,
    },
    /// Mark a task as done
    Done {
        #[arg(help = "The task ID to mark as done")]
        id: usize,
    },
    /// Remove a task
    Remove {
        #[arg(help = "The task ID to remove")]
        id: usize,
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
        Commands::Add { task, due } => {
            if let Some(ref d) = due {
                if NaiveDate::parse_from_str(d, "%Y-%m-%d").is_err() {
                    println!("Invalid due date format. Use YYYY-MM-DD.");
                    return;
                }
            }

            match add_task(&conn, &task, due.as_deref()) {
                Ok(_) => println!("Added task: {}", task),
                Err(e) => println!("Failed to add task: {}", e),
            }
        }

        Commands::List { format } => match get_tasks(&conn) {
            Ok(tasks) => match format {
                OutputFormat::Table => {
                    let mut table = Table::new();

                    table.add_row(row!["#", "Task", "Done", "Due Date"]);

                    for task in tasks {
                        table.add_row(row![
                            task.idx,
                            task.task,
                            if task.done { " ✓✓" } else { " " },
                            task.due.as_deref().unwrap_or(" "),
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

        Commands::Done { id } => {
            if id == 0 {
                println!("Invalid task ID");
                return;
            }

            match mark_done(&conn, id - 1) {
                Ok(_) => println!("Marked task #{} as done", id),
                Err(_) => println!("Task #{} not found", id),
            }
        }

        Commands::Remove { id } => {
            if id == 0 {
                println!("Invalid task ID");
                return;
            }

            match remove_task(&conn, id - 1) {
                Ok(_) => println!("Removed task #{}", id),
                Err(_) => println!("Task #{} not found", id),
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
