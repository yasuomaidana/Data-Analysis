use clap::Parser;
use diesel::associations::HasTable;

use diesel::define_sql_function;
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Text};
use orm_module::establish_connection;
use orm_module::schema::docs_schema::gin_array_docs::dsl as doc_schema;
use orm_module::schema::docs_schema::gin_array_docs::dsl::gin_array_docs;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // see https://crates.io/crates/pgtrgm
    word: Vec<String>,
}

// Define the Postgres function string_to_array so we can call it in Diesel's
// DSL. The first arg is nullable because the `doc` column is Nullable<Text>.
define_sql_function! {
    fn string_to_array(x: Nullable<Text>, sep: Text) -> Array<Text>;
}

fn main() {
    let cli = Cli::parse();
    let mut connection = establish_connection();
    // Convert the `doc` text into a text[] using string_to_array(doc, ' ')
    // then use the array containment operator. We pass the separator as a
    // SQL literal typed as Text.
    let sep = diesel::dsl::sql::<Text>("' '");
    let query = gin_array_docs::table()
        .select((doc_schema::id, doc_schema::doc))
        .filter(string_to_array(doc_schema::doc, sep).contains(&cli.word));
    println!(
        "SQL query: {}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );
    let results = query
        .load::<(i32, Option<String>)>(&mut connection)
        .unwrap_or_else(|e| {
            eprintln!("Query failed: {}", e);
            Vec::new()
        });

    println!("Search words: {:?}", cli.word);
    println!("Results: ({})", results.len());
    for (id, doc) in results {
        println!("id: {}, doc: {:?}", id, doc);
        println!("------");
    }
}
