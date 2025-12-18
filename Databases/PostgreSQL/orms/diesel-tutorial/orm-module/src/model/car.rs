use crate::schema::car_schema::cars;
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = cars)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Car {
    pub id: i32,
    pub model: String,
    pub year: i32,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = cars)]
pub struct NewCar {
    pub model: String,
    pub year: i32,
}

impl NewCar {
    pub fn create(&self, conn: &mut PgConnection) -> Car {
        let mut sanitized = self.clone();
        sanitized.model = sanitized.model.trim().to_string();

        diesel::insert_into(cars::table)
            .values(sanitized)
            .get_result(conn)
            .expect("Error saving new car")
    }
}
