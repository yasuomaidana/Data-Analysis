use crate::schema::sentences_schema::sentences;
use diesel::{Identifiable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug, Identifiable)]
// #[diesel(table_name = cars)] Not needed since the struct name is car
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sentence {
    pub id: i32,
    pub text: String,
}
