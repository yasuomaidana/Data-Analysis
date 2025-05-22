mod commands_implementations;
mod postgres_config;

use crate::commands_implementations::{create_table, download};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct RawCSVCommands {
    #[clap(short, long, value_parser, default_value = ".env")]
    config: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Download { url: String },
    Delete,
    Upload,
    CreateTable,
}

impl RawCSVCommands {
    fn execute(&self) {
        match &self.command {
            Commands::Download { url } => {
                download(url);
            }
            Commands::Delete => {
                println!("Deleting...");
            }
            Commands::Upload => {
                println!("Uploading...");
            }
            Commands::CreateTable => {
                create_table(&self.config);
            }
        }
    }
}

fn main() {
    let cmd = RawCSVCommands::parse();
    cmd.execute();
}
