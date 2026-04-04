use clap::Parser;
use diesel::QueryDsl;
use diesel::query_dsl::RunQueryDsl;

use clap::Subcommand;
use diesel::SelectableHelper;
use orm_module::establish_connection;
use orm_module::model::doc::Doc;
use orm_module::schema::docs_schema::docs::dsl::docs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new doc
    Create {
        /// Doc to create
        #[arg(short, long)]
        doc: String,
    },
    /// Get all docs
    GetAll,
    /// Find doc by key-words
    Find {
        /// Words to search for (space separated)
        #[arg(short, long)]
        words: Vec<String>,
    },
}
fn main() {
    let cli = Cli::parse();
    let mut conn = establish_connection();
    match cli.command {
        Commands::Create { doc, .. } => {
            println!("Creating doc: {}", doc)
        }
        Commands::GetAll => {
            docs.select(Doc::as_select())
                .load(&mut conn)
                .expect("Error loading docs")
                .iter()
                .for_each(|d| println!("{:?}", d));
        }
        Commands::Find { words, .. } => {
            println!("Finding docs with words: {:?}", words)
        }
    };
}
