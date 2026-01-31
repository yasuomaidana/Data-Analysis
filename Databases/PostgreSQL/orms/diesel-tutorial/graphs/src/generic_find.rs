#[macro_use]
mod common;

use clap::{Parser, ValueEnum};
use diesel::connection::LoadConnection;
use diesel::{BoolExpressionMethods, ExpressionMethods, SelectableHelper};
use diesel::{Connection, JoinOnDsl, QueryDsl};
use diesel::{PgConnection, RunQueryDsl, SqliteConnection};
use diesel_cte_ext::{Columns, RecursiveCTEExt, RecursiveParts};
use orm_module::get_url;
use orm_module::model::graph::{AccountRelationshipReturn, EntityReturn};
use orm_module::schema::graph_schema::{account_relationships, entities, relationships};

#[derive(diesel::MultiConnection)]
enum DatabaseConnection {
    Sqlite(SqliteConnection),
    Postgres(PgConnection),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Account id to start traversal from
    #[arg(short, long, default_value = "account_1")]
    account: String,
    #[arg(short, long, default_value = "code-repo")]
    element_type: ElementType,
}

#[derive(Debug, Clone, ValueEnum)]
enum ElementType {
    CodeRepo,
    PullRequest,
}

macro_rules! get_prs {
    ($conn:expr, $backend:ty, $args:expr) => {{
        let element_type = match $args.element_type {
            ElementType::CodeRepo => "CodeRepo",
            ElementType::PullRequest => "PullRequest",
        };
        let user:String = $args.account.to_string();
        // 1. Define the Anchor (The initial SELECT)
        let anchor = entities::table
            .filter(
                entities::_class
                    .eq("Account")
                    .and(entities::id.eq(user.to_string())),
            )
            .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
            .select((
                EntityReturn::as_select(),
                relationships::_class,
                relationships::target_entity_id,
            ));

        // 2. Define the Recursive Term
        let recursive = make_get_recursive!();

        let cols = Columns::for_table::<account_relationships::table>();

        // 3. Define the Final Query
        let final_query = account_relationships::table
            .inner_join(
                entities::table.on(account_relationships::target_entity_id.eq(entities::id)),
            )
            .select((
                AccountRelationshipReturn::as_select(),
                EntityReturn::as_select(),
            ))
            .filter(entities::_class.eq(element_type));

        // see https://docs.rs/crate/diesel-cte-ext/0.1.0/source/src/cte.rs
        let query = $conn.with_recursive_not_all(
            "account_relationships",
            cols,
            RecursiveParts::new(anchor, recursive, final_query),
        );

        println!(
            "Executing query:\n{}",
            diesel::debug_query::<$backend, _>(&query).to_string()
        );
        println!("--\n\n--\n");

        let result = query
            .get_results::<(AccountRelationshipReturn, EntityReturn)>($conn)
            .expect("Failed Recursive query");

        println!("{element_type} from {user}");
        for (ar, er) in result {
            let title = er.metadata.as_ref()
                .and_then(|t| t.get("name").and_then(|v| v.as_str())).unwrap_or(er.id.as_str()).to_string();
            println!("{:?}[{:?}]->{:?}", ar.id, ar._type,title);
        }
    }};
}

fn main() {
    let args = Cli::parse();

    let database_url = get_url();
    let mut connection =
        DatabaseConnection::establish(&database_url).expect("Error connecting to database");
    match &mut connection {
        DatabaseConnection::Sqlite(db_connection) => {
            get_prs!(db_connection, diesel::sqlite::Sqlite, args)
        }
        DatabaseConnection::Postgres(db_connection) => get_prs!(db_connection, diesel::pg::Pg, args),
    }
}
