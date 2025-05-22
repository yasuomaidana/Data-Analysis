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
    Delete {
        #[clap(short, long, default_value_t = false)]
        drop: bool
    },
    Upload,
    CreateTable,
}

impl RawCSVCommands {
    fn execute(&self) {
        match &self.command {
            Commands::Download { url } => {
                download(url);
            }
            Commands::Delete{drop} => {
                println!("Deleting...");
            }
            Commands::Upload => {
                println!("Uploading...");
                let database_config = postgres_config::read_conf_from_env(&self.config);
                let csv_file_path = "library.csv";
                commands_implementations::copy_csv_to_postgres(database_config, csv_file_path);
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
