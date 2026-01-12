use diesel::pg::Pg;
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

fn get_prs_pg(db_connection: &mut PgConnection) {
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
        .into_boxed::<Pg>();

    let recursive = account_relationships::table
        .inner_join(entities::table.on(account_relationships::target_entity_id.eq(entities::id)))
        .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
        .select((
            AccountRelationshipReturn::as_select(),
            EntityReturn::as_select(),
        ))
        .into_boxed::<Pg>();

    let cols = Columns::for_table::<account_relationships::table>();

    let final_query = account_relationships::table
        .inner_join(entities::table.on(account_relationships::target_entity_id.eq(entities::id)))
        .distinct()
        .select((
            AccountRelationshipReturn::as_select(),
            EntityReturn::as_select(),
        ))
        .filter(entities::_class.eq("CodeRepo"))
        .into_boxed::<Pg>();

    let query = db_connection.with_recursive(
        "account_relationships",
        cols,
        RecursiveParts::new(anchor, recursive, final_query),
    );

    let result = query
        .load::<(AccountRelationshipReturn, EntityReturn)>(db_connection)
        .expect("Failed Recursive query (Pg)");

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
    let mut connection =
        DatabaseConnection::establish(&database_url).expect("Error connecting to database");
    match &mut connection {
        DatabaseConnection::Sqlite(db_connection) => get_prs_sqlite(db_connection),
        DatabaseConnection::Postgres(db_connection) => get_prs_pg(db_connection),
    }
}
