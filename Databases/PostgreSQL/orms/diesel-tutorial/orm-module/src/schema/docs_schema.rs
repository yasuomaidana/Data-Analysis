// @generated automatically by Diesel CLI.

diesel::table! {
    doc_gin (key_word, doc_id) {
        key_word -> Text,
        doc_id -> Int4,
    }
}

diesel::table! {
    docs (id) {
        id -> Int4,
        doc -> Text,
    }
}

diesel::table! {
    stem_words (id) {
        id -> Int4,
        word -> Text,
        stem -> Text,
    }
}

diesel::table! {
    stop_words (id) {
        id -> Int4,
        word -> Text,
    }
}

diesel::joinable!(doc_gin -> docs (doc_id));

diesel::allow_tables_to_appear_in_same_query!(doc_gin, docs, stem_words, stop_words,);
