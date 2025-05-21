mod commands_implementations;

use std::path::PathBuf;
use crate::commands_implementations::download;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct RawCSVCommands {
    #[clap(short, long, value_parser, default_value = ".env")]
    config:PathBuf,
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

impl Commands {
    fn execute(&self) {
        match self {
            Commands::Download { url } => download(url),
            Commands::Delete => println!("Deleting..."),
            Commands::Upload => println!("Uploading..."),
            Commands::CreateTable => println!("Creating table..."),
        }
    }
}

fn main() {
    let cmd = RawCSVCommands::parse();
    cmd.command.execute();
}
