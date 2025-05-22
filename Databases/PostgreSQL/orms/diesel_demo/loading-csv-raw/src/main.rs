mod commands_implementations;
mod postgres_config;

use crate::commands_implementations::{clear_table, create_table, download, drop_table, show_track_raw};
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
        #[clap(short, long, default_value_t = false, help = "Drop the table")]
        drop: bool
    },
    Upload,
    CreateTable,
    Read,
}

impl RawCSVCommands {
    fn execute(&self) {
        match &self.command {
            Commands::Download { url } => {
                download(url);
            }
            Commands::Delete{drop} => {
                println!("Deleting...");
                let database_config = postgres_config::read_conf_from_env(&self.config);
                if *drop {
                    drop_table(&database_config);
                }else{
                    clear_table(&database_config);
                }
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
            Commands::Read => {
                let database_config = postgres_config::read_conf_from_env(&self.config);
                show_track_raw(&database_config);
            }
        }
    }
}

fn main() {
    let cmd = RawCSVCommands::parse();
    cmd.execute();
}
