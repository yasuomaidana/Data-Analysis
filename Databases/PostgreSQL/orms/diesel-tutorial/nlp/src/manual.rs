use clap::Parser;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_query;
use diesel::sql_types::{Array, Text};

use clap::Subcommand;
use diesel::SelectableHelper;

// diesel::dsl::not/select are not needed for get-keywords; keep imports minimal
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
            let query = r#"
WITH input(word) AS (
  SELECT unnest($1::text[])
)
SELECT COALESCE(stem_words.stem, input.word) AS keyword
FROM input
LEFT JOIN stop_words ON stop_words.word = input.word
LEFT JOIN stem_words ON stem_words.word = input.word
WHERE stop_words.word IS NULL
"#;

            #[derive(QueryableByName)]
            struct KeywordRow {
                #[diesel(sql_type = Text)]
                keyword: String,
            }

            let rows: Vec<KeywordRow> = sql_query(query)
                .bind::<Array<Text>, _>(lower_words)
                .load(&mut conn)
                .expect("Error executing keyword query");

            let keywords: Vec<String> = rows.into_iter().map(|r| r.keyword).collect();

            println!("Keywords: {:?}", keywords);
        }
    };
}
