// @generated automatically by Diesel CLI.

diesel::table! {
    jtrack (id) {
        id -> Int4,
        body -> Nullable<Jsonb>,
    }
}
