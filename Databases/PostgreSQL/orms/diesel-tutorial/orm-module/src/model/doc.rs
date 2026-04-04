use crate::schema::docs_schema::docs;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Identifiable, Selectable, Queryable, Debug, Insertable)]
pub struct Doc {
    pub id: i32,
    pub doc: String,
}
