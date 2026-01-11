use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};
use diesel::{JoinOnDsl, QueryDsl};
use diesel_cte_ext::{Columns, RecursiveCTEExt, RecursiveParts};
use orm_module::establish_connection;
use orm_module::model::graph::{
    AccountRelationship, AccountRelationshipReturn, Entity, Relationship,
};
use orm_module::schema::graph_schema::{account_relationships, entities, relationships};

fn main() {
    let mut conn = establish_connection();

    // 1. Define the Anchor (The initial SELECT)
    let anchor = entities::table
        .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
        .select((
            Entity::as_select(),
            relationships::_class,
            relationships::target_entity_id,
        ));

    // 2. Define the Recursive Term
    let recursive = entities::table
        .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
        .inner_join(
            account_relationships::table
                .on(account_relationships::target_entity_id.eq(entities::id)),
        )
        .select((
            Entity::as_select(),
            relationships::_class,
            relationships::target_entity_id,
        ));

    let cols = Columns::for_table::<account_relationships::table>();

    // 3. Define the Final Query
    let final_query = account_relationships::table
        .inner_join(entities::table.on(account_relationships::target_entity_id.eq(entities::id)))
        .distinct()
        .select((AccountRelationshipReturn::as_select(), Entity::as_select()));

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
        .get_results::<(AccountRelationshipReturn, Entity)>(&mut conn)
        .expect("Failed Recursive query");

    for i in result {
        println!("{:?}", i);
    }
}
