use clap::{Parser, Subcommand};
use diesel::dsl::update;
use diesel::{
    ExpressionMethods, PgConnection, PgJsonbExpressionMethods, QueryDsl, RunQueryDsl, debug_query,
};
use json::{JTrack, Track};
use orm_module::establish_connection;
use orm_module::schema::jtrack_schema::jtrack;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    /// Use the containment operator ->> to find rows where body contains {"name":"Summer Nights"}
    Update {
        /// Use `--favorite` to set favorite = true (defaults to false)
        #[arg(long, default_value_t = false)]
        favorite: bool,
    },
    Show,
    ShowSimple,
}

fn show(conn: &mut PgConnection) {
    let query = jtrack::table
        .filter(Track::count_sql().gt(200))
        .select(jtrack::body);

    println!("SQL: {}", debug_query::<diesel::pg::Pg, _>(&query));

    // `jtrack::body` is Nullable<Jsonb>, so load as Option<Value>
    let tracks = query
        .load::<Option<Value>>(conn)
        .expect("Failed to load jtrack entries");

    if tracks.is_empty() || tracks.iter().all(|t| t.is_none()) {
        println!("No tracks found");
    }

    for track_opt in tracks {
        match track_opt {
            Some(track) => println!("{:?}", track),
            None => println!("null"),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let mut connection = establish_connection();
    match cli.command {
        Command::Update { favorite } => {
            let query = update(jtrack::table)
                .filter(Track::count_sql().gt(200))
                .set(
                    jtrack::body.eq(jtrack::body.concat(serde_json::json!({"favorite": favorite}))),
                );
            println!("SQL: {}", debug_query::<diesel::pg::Pg, _>(&query));
            query
                .execute(&mut connection)
                .expect("Failed to update jtrack entries");
            show(&mut connection);
        }
        Command::Show => {
            show(&mut connection);
        }
        Command::ShowSimple => {
            let query = jtrack::table.filter(Track::count_sql().gt(200));
            println!("SQL: {}", debug_query::<diesel::pg::Pg, _>(&query));
            let raw_bodies = query
                .load::<JTrack>(&mut connection)
                .expect("Failed to load jtrack entries");
            if raw_bodies.is_empty() {
                println!("No tracks found");
            }
            raw_bodies.iter().for_each(|b| {
                println!("{:?}", b.body);
            })
        }
    }
}
