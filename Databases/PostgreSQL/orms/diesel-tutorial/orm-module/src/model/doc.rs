use crate::schema::docs_schema::{docs, stem_words, stop_words};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Identifiable, Selectable, Queryable, Debug, Insertable)]
#[diesel(table_name = docs)]
pub struct Doc {
    pub id: i32,
    pub doc: String,
}

#[derive(Identifiable, Selectable, Queryable, Debug, Insertable)]
#[diesel(table_name = stem_words)]
pub struct StemWord {
    pub id: i32,
    pub word: String,
    pub stem: String,
}

#[derive(Identifiable, Selectable, Queryable, Debug, Insertable)]
#[diesel(table_name = stop_words)]
pub struct StopWord {
    pub id: i32,
    pub word: String,
}
