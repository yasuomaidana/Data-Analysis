use clap::Parser;
use diesel::pg::Pg;
use diesel::{QueryDsl, RunQueryDsl};
use diesel_full_text_search::*;
use orm_module::establish_connection;
use orm_module::schema::docs_schema::gin_ts_docs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // see https://crates.io/crates/pgtrgm
    ts_query: String,

    #[arg(short, long, default_value_t = false)]
    spanish: bool,
}

use diesel_full_text_search::configuration::TsConfiguration;

fn main() {
    let cli = Cli::parse();
    let mut conn = establish_connection();

    // Box the query so both branches can mutate the same concrete type.
    let mut query = gin_ts_docs::table.into_boxed::<Pg>();

    if cli.spanish {
        query = query.filter(
            to_tsvector_with_search_config(TsConfiguration::SPANISH, gin_ts_docs::doc).matches(
                to_tsquery_with_search_config(&TsConfiguration::SPANISH, &cli.ts_query),
            ),
        );
    } else {
        query = query.filter(to_tsvector(gin_ts_docs::doc).matches(to_tsquery(&cli.ts_query)));
    }

    println!(
        "SQL query: {}",
        diesel::debug_query::<Pg, _>(&query).to_string()
    );
    let results = query.load::<(i32, Option<String>)>(&mut conn);
    match results {
        Ok(results) => {
            if results.is_empty() {
                println!("No results found");
                return;
            }
            println!("Results: ({})", results.len());
            for (id, doc) in results {
                println!("id: {}, doc: {:?}", id, doc);
                println!("------");
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
