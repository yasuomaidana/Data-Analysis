use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::car_schema::cars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Car {
    pub id: i32,
    pub model: String,
    pub year: i32,
}
