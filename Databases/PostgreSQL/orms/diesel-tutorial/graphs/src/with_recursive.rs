use diesel::ExpressionMethods;
use diesel::dsl::{select, sql};
use diesel::pg::Pg;
use diesel::sql_types::Integer;
use diesel::{JoinOnDsl, QueryDsl};
use diesel_cte_ext::{Columns, RecursiveCTEExt, RecursiveParts};
use orm_module::establish_connection;
use orm_module::schema::graph_schema::{entities, relationships};

diesel::table! {
    account_relationships (id) {
        id -> Text,
        _type -> Text,
        metadata -> Nullable<Jsonb>,
        relationship_class -> Nullable<Text>,
        target_entity_id -> Text,
    }
}

fn main() {
    let mut conn = establish_connection();

    // 1. Define the Anchor (The initial SELECT)
    // SELECT *, r._class AS relationship_class FROM public.entity e JOIN public.relationship r ON e.id = r.source_entity_id
    let anchor = entities::table
        .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
        .select((
            entities::id,
            entities::_type,
            entities::metadata,
            relationships::_class,
            relationships::target_entity_id,
        ));

    let cols = account_relationships::all_columns;

    println!("{:?}", cols);

    println!("{:?}", account_relationships::all_columns);
    let cols = Columns::for_table::<account_relationships::table>();

    let a = conn.with_recursive(
        "account_relationships",
        cols,
        RecursiveParts::new(
            anchor,
            account_relationships::table,
            sql::<Integer>("SELECT n FROM series"),
        ),
    );

    println!("{}", diesel::debug_query::<Pg, _>(&a).to_string());
}
