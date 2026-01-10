use crate::schema::graph_schema::{entities, relationships};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

#[derive(Identifiable, Selectable, Queryable, Debug, Insertable)]
#[diesel(table_name = entities)]
pub struct Entity {
    pub id: String,
    pub _type: String,
    pub _class: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[diesel(belongs_to(Entity, foreign_key = source_entity_id))]
#[diesel(primary_key(source_entity_id, target_entity_id))]
pub struct Relationship {
    pub source_entity_id: String,
    pub target_entity_id: String,
    pub _class: Option<String>,
}
