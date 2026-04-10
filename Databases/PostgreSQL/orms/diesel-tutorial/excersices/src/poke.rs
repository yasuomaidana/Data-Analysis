use clap::{Parser, Subcommand};
use diesel::dsl::count_star;
use diesel::{Connection, ConnectionResult, PgConnection, QueryDsl, RunQueryDsl, sql_query};
use excersices::{build_database_url, impl_get_connection};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    host: String,
    port: u16,
    database: String,
    user: String,
    password: String,

    #[command(subcommand)]
    command: Commands,
}

impl_get_connection!(Args);

#[derive(Debug, Subcommand)]
enum Commands {
    // Creates the pokeapi table if it doesn't exits
    Create,
    // Fill the pokeapi table with the first 100 items if they are not loaded yet
    Fill,
    // Shows the stored items
    Get,
}

diesel::table! {
    pokeapi {
        id -> Integer,
        body -> Jsonb,
    }
}

fn show_poke_api(conn: &mut PgConnection) {
    let pokes: Vec<Value> = pokeapi::table
        .select(pokeapi::body)
        .limit(2)
        .load::<Value>(conn)
        .expect("Failed to load pokes");
    for poke in pokes {
        println!("{}", poke);
    }
}

fn main() {
    let args = Args::parse();
    let mut conn = args
        .get_connection()
        .expect("Failed to connect to the database");
    match args.command {
        Commands::Create => {
            sql_query("CREATE TABLE IF NOT EXISTS pokeapi (id SERIAL PRIMARY KEY, body JSONB);")
                .execute(&mut conn)
                .expect("Failed to create table");
        }
        Commands::Fill => {
            if pokeapi::table
                .select(count_star())
                .first::<i64>(&mut conn)
                .unwrap_or(0)
                == 0
            {
                #[derive(diesel::Insertable)]
                #[diesel(table_name = pokeapi)]
                struct NewPoke {
                    body: Value,
                }

                for i in 0..100 {
                    let id = i + 1;
                    println!("Filling {id}");
                    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
                    if let Ok(resp) = reqwest::blocking::get(&url) {
                        if let Ok(json_body) = resp.json::<Value>() {
                            let new_row = NewPoke { body: json_body };
                            diesel::insert_into(pokeapi::table)
                                .values(&new_row)
                                .execute(&mut conn)
                                .expect("Failed to insert value");
                        }
                    }
                }
                show_poke_api(&mut conn);
            }
        }
        Commands::Get => {
            show_poke_api(&mut conn);
        }
    }
}
