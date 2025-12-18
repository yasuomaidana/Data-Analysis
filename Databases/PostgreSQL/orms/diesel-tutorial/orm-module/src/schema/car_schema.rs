// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Int4,
        #[max_length = 50]
        model -> Varchar,
        year -> Int4,
    }
}
