mod common;
use crate::common::get_recursive;
use diesel::sqlite::Sqlite;
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

fn get_prs_pg(conn: &mut PgConnection) {
    // 1. Define the Anchor (The initial SELECT)
    let anchor = entities::table
        .filter(
            entities::_class
                .eq("Account")
                .and(entities::id.eq("account_1")),
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
        .get_results::<(AccountRelationshipReturn, EntityReturn)>(conn)
        .expect("Failed Recursive query");

    for i in result {
        println!("{:?}", i);
    }
}

fn get_prs_sqlite(db_connection: &mut SqliteConnection) {
    let anchor = account_relationships::table
        .inner_join(entities::table.on(account_relationships::target_entity_id.eq(entities::id)))
        .filter(
            entities::_class
                .eq("Account")
                .and(entities::id.eq("account_1")),
        )
        .select((
            AccountRelationshipReturn::as_select(),
            EntityReturn::as_select(),
        ))
        .into_boxed::<Sqlite>();

    let recursive = account_relationships::table
        .inner_join(entities::table.on(account_relationships::target_entity_id.eq(entities::id)))
        .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
        .select((
            AccountRelationshipReturn::as_select(),
            EntityReturn::as_select(),
        ))
        .into_boxed::<Sqlite>();

    let cols = Columns::for_table::<account_relationships::table>();

    let final_query = account_relationships::table
        .inner_join(entities::table.on(account_relationships::target_entity_id.eq(entities::id)))
        .distinct()
        .select((
            AccountRelationshipReturn::as_select(),
            EntityReturn::as_select(),
        ))
        .filter(entities::_class.eq("CodeRepo"))
        .into_boxed::<Sqlite>();

    let query = db_connection.with_recursive(
        "account_relationships",
        cols,
        RecursiveParts::new(anchor, recursive, final_query),
    );

    let result = query
        .load::<(AccountRelationshipReturn, EntityReturn)>(db_connection)
        .expect("Failed Recursive query (Sqlite)");

    for i in result {
        println!("{:?}", i);
    }
}

fn main() {
    let database_url = get_url();
    // TODO Create macro and check sqlite logic
    let mut connection =
        DatabaseConnection::establish(&database_url).expect("Error connecting to database");
    match &mut connection {
        DatabaseConnection::Sqlite(db_connection) => get_prs_sqlite(db_connection),
        DatabaseConnection::Postgres(db_connection) => get_prs_pg(db_connection),
    }
}
