mod config;
mod db;

use config::*;
use db::*;

use chrono::{NaiveDate, NaiveDateTime};
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

        #[arg(long, help = "Due date (YYYY-MM-DD or YYYY-MM-DD HH:MM)")]
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
    let config = load_config();

    let db_path = get_data_path();
    let conn = init_db(&db_path).expect("Failed to initialize database");

    match cli.command {
        Commands::Add { task, due } => {
            let formatted_due = match due {
                Some(ref d) => match parse_due(d) {
                    Some(validated) => Some(validated),
                    None => {
                        println!("Invalid due date format. Use YYYY-MM-DD or YYYY-MM-DD HH:MM.");
                        return;
                    }
                },
                None => None,
            };

            match add_task(&conn, &task, formatted_due.as_deref()) {
                Ok(_) => println!("Added task: {}", task),
                Err(e) => eprintln!("Failed to add task: {}", e),
            }
        }

        Commands::List { format } => match get_tasks(&conn) {
            Ok(tasks) => match format {
                OutputFormat::Table => {
                    let table =
                        get_pretty_table(&tasks, &config.time_format.unwrap_or(TimeFormat::H24));

                    table.printstd();
                }
                OutputFormat::Json => {
                    let json =
                        serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks");

                    println!("{}", json);
                }
            },
            Err(e) => eprintln!("Failed to list tasks: {}", e),
        },

        Commands::Done { id } => {
            if id == 0 {
                println!("Invalid task ID");
                return;
            }

            match mark_done(&conn, id - 1) {
                Ok(_) => println!("Marked task #{} as done", id),
                Err(_) => eprintln!("Task #{} not found", id),
            }
        }

        Commands::Remove { id } => {
            if id == 0 {
                println!("Invalid task ID");
                return;
            }

            match remove_task(&conn, id - 1) {
                Ok(_) => println!("Removed task #{}", id),
                Err(_) => eprintln!("Task #{} not found", id),
            }
        }

        Commands::Clean => match clear_tasks(&conn) {
            Ok(_) => println!("Cleared completed tasks"),
            Err(e) => eprintln!("Failed to clear completed tasks: {}", e),
        },

        Commands::Reset => {
            remove_data_dir();
        }
    }
}

fn format_due_date(due_opt: Option<&str>, time_format: &TimeFormat) -> String {
    match due_opt {
        Some(due_str) => {
            if let Ok(dt) = NaiveDateTime::parse_from_str(due_str, "%Y-%m-%d %H:%M") {
                match time_format {
                    TimeFormat::H24 => dt.format("%Y-%m-%d %H:%M").to_string(),
                    TimeFormat::H12 => dt.format("%m/%d/%Y %I:%M %p").to_string(),
                }
            } else if let Ok(date) = NaiveDate::parse_from_str(due_str, "%Y-%m-%d") {
                match time_format {
                    TimeFormat::H24 => date.format("%Y-%m-%d").to_string(),
                    TimeFormat::H12 => date.format("%m/%d/%Y").to_string(),
                }
            } else {
                due_str.to_string()
            }
        }
        None => " ".to_string(),
    }
}

fn get_pretty_table(tasks: &[Task], time_format: &TimeFormat) -> Table {
    let mut table = Table::new();
    table.add_row(row!["#", "Task", "Done", "Due Date"]);

    for task in tasks {
        let due_str = format_due_date(task.due.as_deref(), time_format);

        table.add_row(row![
            task.idx,
            task.task,
            if task.done { " ✓✓" } else { " " },
            due_str,
        ]);
    }

    table
}

fn parse_due(due: &str) -> Option<String> {
    if let Ok(date_time) = NaiveDateTime::parse_from_str(due, "%Y-%m-%d %H:%M") {
        return Some(date_time.format("%Y-%m-%d %H:%M").to_string());
    }

    if let Ok(date) = NaiveDate::parse_from_str(due, "%Y-%m-%d") {
        return Some(date.format("%Y-%m-%d").to_string());
    }

    None
}
