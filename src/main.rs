mod commands;
mod models;
mod storage;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "silver-tasks")]
#[command(about = "Personal task manager for university students and productivity", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task name/description (1-200 characters)
        name: String,
        /// Tag for categorization
        tag: String,
        /// Optional deadline (DD.MM.YYYY, YYYY-MM-DD, or DD/MM/YYYY)
        deadline: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            name,
            tag,
            deadline,
        } => {
            commands::add::add_task(name, tag, deadline)?;
        }
    }

    Ok(())
}

