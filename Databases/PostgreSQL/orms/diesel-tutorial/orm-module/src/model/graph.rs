use crate::schema::graph_schema::entities;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Identifiable, Selectable, Queryable, Debug, Insertable)]
#[diesel(table_name = entities)]
pub struct Entity {
    pub id: String,
    pub _type: String,
    pub _class: String,
    pub metadata: Option<serde_json::Value>,
}
