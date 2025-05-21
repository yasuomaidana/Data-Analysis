mod commands_implementations;

use crate::commands_implementations::download;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct RawCSVCommands {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Download {
        url: String,
    },
    Delete,
    Upload,
}

impl Commands {
    fn execute(&self) {
        match self {
            Commands::Download { url } => download(url),
            Commands::Delete => println!("Deleting..."),
            Commands::Upload => println!("Uploading..."),
        }
    }
}

fn main() {
    let cmd = RawCSVCommands::parse();
    cmd.command.execute();
}
