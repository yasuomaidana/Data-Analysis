use diesel::ExpressionMethods;
use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::{JoinOnDsl, QueryDsl, SelectableHelper};
use orm_module::model::graph::EntityReturn;
use orm_module::schema::graph_schema::{account_relationships, entities, relationships};

pub fn get_recursive() -> impl QueryFragment<Pg> {
    entities::table
        .inner_join(relationships::table.on(entities::id.eq(relationships::source_entity_id)))
        .inner_join(
            account_relationships::table
                .on(account_relationships::target_entity_id.eq(entities::id)),
        )
        .select((
            EntityReturn::as_select(),
            relationships::_class,
            relationships::target_entity_id,
        ))
}
