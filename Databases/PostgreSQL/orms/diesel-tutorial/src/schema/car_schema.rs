// @generated automatically by Diesel CLI.

diesel::table! {
    car (id) {
        id -> Int4,
        #[max_length = 50]
        model -> Varchar,
        year -> Int4,
    }
}
