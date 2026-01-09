// @generated automatically by Diesel CLI.

use crate::schema::user_schema::users;

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pages (id) {
        id -> Int4,
        page_number -> Int4,
        content -> Text,
        book_id -> Int4,
    }
}

diesel::joinable!(books -> users (author_id));
diesel::joinable!(pages -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(books, pages, users);
