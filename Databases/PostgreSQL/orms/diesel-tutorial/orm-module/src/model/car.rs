use crate::schema::car_schema::cars;
use diesel::{
    AsChangeset, Identifiable, Insertable, PgConnection, Queryable, RunQueryDsl, Selectable,
};

/// Diesel derives used on `Car`:
/// - `Queryable` — convert DB rows into `Car` instances (e.g. `cars::table.load::<Car>(&conn)`).
/// - `Selectable` — allow `Car` to be used as a selectable projection in queries (`Car::as_select()`).
/// - `Identifiable` — marks `id` as the primary key for Diesel helpers (used for updates/deletes).
/// - `AsChangeset` — lets `Car` be used as an update payload in `diesel::update(...).set(&car)`.
/// - `Debug` — enables `{:?}` formatting for debugging.
#[derive(Queryable, Selectable, Debug, Identifiable, AsChangeset)]
// #[diesel(table_name = cars)] Not needed since the struct name is car
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
