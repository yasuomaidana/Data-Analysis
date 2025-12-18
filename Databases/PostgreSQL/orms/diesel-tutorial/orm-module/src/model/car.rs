use crate::schema::car_schema::cars;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = cars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Car {
    pub id: i32,
    pub model: String,
    pub year: i32,
}

#[derive(Insertable)]
#[diesel(table_name = cars)]
pub struct NewCar {
    pub model: String,
    pub year: i32,
}
