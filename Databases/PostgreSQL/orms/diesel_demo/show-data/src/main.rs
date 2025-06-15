mod commands;

use crate::commands::Commands;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct ShowDataCommands {
    #[clap(short, long, value_parser, default_value = ".env")]
    config: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let commands = ShowDataCommands::parse();
    match &commands.command {
        Commands::Albums => println!("Listing albums..."),
        Commands::Artist => println!("Showing artist..."),
        Commands::Tracks => println!("Listing tracks..."),
        Commands::TrackToArtists => println!("Mapping tracks to artists..."),
        Commands::GetTables(args) => {
            println!("Getting table information for: {:?}", args.tables);
        }
    }
}
