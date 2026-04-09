use diesel::{Queryable, Selectable};
use diesel_json::Json;
use orm_module::schema::jtrack_schema::jtrack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub album: String,
    pub count: u32,
    pub artist: String,
    /// length in milliseconds
    pub length: u64,
    pub rating: u8,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = jtrack)]
pub struct JTrack {
    pub id: i32,
    pub body: Option<Json<Track>>,
}
