use clap::{Parser, Subcommand};
use diesel::PgJsonbExpressionMethods;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use orm_module::establish_connection;
use orm_module::schema::jtrack_schema::jtrack::dsl::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    /// Use the containment operator @> to find rows where body contains {"name":"Summer Nights"}
    Arrow {
        #[arg(default_value = "Summer Nights")]
        search: String,
    },
    /// Use the ->> operator in WHERE to match name text
    Contains {
        #[arg(default_value = "Summer Nights")]
        search: String,
    },
    /// Count rows where the top-level key 'favorite' exists (jsonb ? 'favorite')
    ContainsKey,
    GetNames,
}

fn show_number_results(results: Vec<Option<String>>) {
    if results.is_empty() {
        println!("No results");
    } else {
        println!("Results ({}):", results.len());
        for res in results.into_iter().flatten() {
            println!("count: {}", res);
        }
    }
}
fn main() {
    let args = Cli::parse();
    let mut conn = establish_connection();

    match args.command {
        Command::Arrow { search } => {
            // SELECT (body->>'count')::int FROM jtrack WHERE body @> '{"name": "Summer Nights"}';
            let results: Vec<Option<String>> = jtrack
                .filter(body.contains(serde_json::json!({"name": search})))
                .select(body.retrieve_as_text("count"))
                .load(&mut conn)
                .expect("Error executing arrow query");

            show_number_results(results);
        }
        Command::Contains { search } => {
            // SELECT (body->>'count')::int FROM jtrack WHERE body->>'name' = 'Summer Nights';
            let results: Vec<Option<String>> = jtrack
                .filter(body.retrieve_as_text("name").eq(search))
                .select(body.retrieve_as_text("count"))
                .load(&mut conn)
                .expect("Error executing contains query");

            show_number_results(results);
        }
        Command::ContainsKey => {
            // SELECT COUNT(*) FROM jtrack WHERE body ? 'favorite';
            let count: i64 = jtrack
                .filter(body.has_key("favorite"))
                .count()
                .get_result(&mut conn)
                .expect("Error executing contains key query");

            println!("rows with 'favorite' key: {}", count);
        }
        Command::GetNames => {
            // SELECT (body->>'name')::text FROM jtrack;
            let results: Vec<Option<String>> = jtrack
                .select(body.retrieve_as_text("name"))
                .load(&mut conn)
                .expect("Error executing get names query");
            results
                .iter()
                .for_each(|name| println!("name: {}", name.as_deref().unwrap_or("NULL")));
        }
    }
}
