macro_rules! make_get_recursive {
    () => {
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
    };
}