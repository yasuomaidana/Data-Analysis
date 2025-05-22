use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct MainCommand {
    #[clap(short, long, value_parser, default_value = ".env")]
    config_file: PathBuf,
}

fn main() {
    let cmd = MainCommand::parse();
    let config_file = cmd.config_file;
    let config_content = std::fs::read_to_string(config_file)
        .expect("Failed to read the config file");
    println!("Config file content:\n{}", config_content);
    
}
