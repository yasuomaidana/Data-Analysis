use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Albums,
    Artist,
    Tracks,
    TrackToArtists,
    GetTables(GetTableInfoArgs),
}
#[derive(Args, Debug)]
pub struct GetTableInfoArgs {
    /// Name(s) of the table(s) to get information about. It can be used multiple times.
    #[arg(short, long, value_name = "TABLE", num_args = 1.., action = clap::ArgAction::Append)]
    pub tables: Vec<String>,
}
