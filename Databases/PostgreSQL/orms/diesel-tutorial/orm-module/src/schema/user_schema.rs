// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    user_configs (id) {
        id -> Int4,
        user_id -> Int4,
        enabled -> Bool,
        points -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        role -> UserRole,
        birth_date -> Nullable<Date>,
        updated -> Timestamp,
    }
}

diesel::joinable!(user_configs -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(user_configs, users,);
