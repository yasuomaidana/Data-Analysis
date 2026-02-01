// @generated automatically by Diesel CLI.

diesel::table! {
    entities (id) {
        id -> Text,
        _type -> Text,
        _class -> Text,
        metadata -> Nullable<Jsonb>,
    }
}

diesel::table! {
    relationships (source_entity_id, target_entity_id) {
        source_entity_id -> Text,
        target_entity_id -> Text,
        _class -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(entities, relationships);
