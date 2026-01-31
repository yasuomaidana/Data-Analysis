mod common;

use crate::common::get_recursive;
use clap::Parser;
use diesel::{BoolExpressionMethods, ExpressionMethods, RunQueryDsl, SelectableHelper};
use diesel::{JoinOnDsl, QueryDsl};
use diesel_cte_ext::{Columns, RecursiveCTEExt, RecursiveParts};
use orm_module::establish_connection;
use orm_module::model::graph::{AccountRelationshipReturn, EntityReturn};
use orm_module::schema::graph_schema::{account_relationships, entities, relationships};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Account id to start traversal from
    #[arg(short, long, default_value = "account_1")]
    account: String,
}
fn main() {
    let mut conn = establish_connection();

    let args = Cli::parse();

    // 1. Define the Anchor (The initial SELECT)
    let anchor = entities::table
        .filter(
            entities::_class
                .eq("Account")
                .and(entities::id.eq(args.account)),
        )
        .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
        .select((
            EntityReturn::as_select(),
            relationships::_class,
            relationships::target_entity_id,
        ));

    // 2. Define the Recursive Term
    let recursive = get_recursive();

    let cols = Columns::for_table::<account_relationships::table>();

    // 3. Define the Final Query
    let final_query = account_relationships::table
        .inner_join(entities::table.on(account_relationships::target_entity_id.eq(entities::id)))
        .distinct()
        .select((
            AccountRelationshipReturn::as_select(),
            EntityReturn::as_select(),
        ))
        .filter(entities::_class.eq("CodeRepo"));

    // see https://docs.rs/crate/diesel-cte-ext/0.1.0/source/src/cte.rs
    let query = conn.with_recursive(
        "account_relationships",
        cols,
        RecursiveParts::new(anchor, recursive, final_query),
    );

    println!(
        "Executing query:\n{}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let result = query
        .get_results::<(AccountRelationshipReturn, EntityReturn)>(&mut conn)
        .expect("Failed Recursive query");

    print!("--\n\n::");
    for i in result {
        println!("{:?}", i);
    }
}
