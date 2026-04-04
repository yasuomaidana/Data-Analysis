use clap::Parser;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::query_dsl::RunQueryDsl;
use std::collections::HashSet;

use clap::Subcommand;
use diesel::SelectableHelper;

// diesel::dsl::not/select are not needed for get-keywords; keep imports minimal
use orm_module::establish_connection;
use orm_module::model::doc::Doc;
use orm_module::schema::docs_schema::docs::dsl::docs;
use orm_module::schema::docs_schema::{stem_words, stop_words};

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
        // #[arg(short, long)]
        words: Vec<String>,
    },
    /// Get a list of words that are keywords
    GetKeywords { words: Vec<String> },
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
        Commands::GetKeywords { words } => {
            let lower_words: Vec<String> = words.into_iter().map(|w| w.to_lowercase()).collect();

            // Build keywords entirely in SQL to avoid Rust-side filtering.
            // We pass the list of lower-cased words as a Postgres text[] parameter,
            // unnest it into rows, left join to stop_words and stem_words, then
            // select the stem when present or the original word otherwise.
            // This returns only words that are NOT stop words.
            // let query = r#"
            // WITH input(word) AS (
            //   SELECT unnest($1::text[])
            // )
            // SELECT COALESCE(stem_words.stem, input.word) AS keyword
            // FROM input
            // LEFT JOIN stop_words ON stop_words.word = input.word
            // LEFT JOIN stem_words ON stem_words.word = input.word
            // WHERE stop_words.word IS NULL
            // "#;
            // Diesel-only approach without creating helper tables or using raw SQL.
            // We query the DB for which of the provided words are stop words and which
            // have stems, then perform the removal and replacement in-memory.
            // This keeps all lookups in SQL (via Diesel DSL) and avoids persisting the input words.

            // Fetch stop words that match any of the input words
            let stops: HashSet<String> = stop_words::table
                .filter(stop_words::word.eq_any(&lower_words))
                .select(stop_words::word)
                .load::<String>(&mut conn)
                .expect("Error loading stop words")
                .into_iter()
                .collect();

            // Fetch stems for words that have them
            let stems_vec: Vec<(String, String)> = stem_words::table
                .filter(stem_words::word.eq_any(&lower_words))
                .select((stem_words::word, stem_words::stem))
                .load(&mut conn)
                .expect("Error loading stem words");

            let stems_map: std::collections::HashMap<String, String> =
                stems_vec.into_iter().collect();

            // Remove stop words (DB-driven detection) and replace with stems when available
            let keywords: Vec<String> = lower_words
                .into_iter()
                .filter(|w| !stops.contains(w))
                .map(|w| stems_map.get(&w).cloned().unwrap_or(w))
                .collect();

            println!("Keywords: {:?}", keywords);
        }
    };
}
