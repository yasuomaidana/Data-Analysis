use diesel::prelude::*;
use orm_module::establish_connection;
use orm_module::model::car::Car;
use orm_module::schema::car_schema::cars::dsl::cars;
use orm_module::schema::car_schema::cars::year;

fn main() {
    let connection = &mut establish_connection();
    let results = cars
        .filter(year.gt(2015))
        .limit(5)
        .select(Car::as_select())
        .load(connection)
        .expect("Error loading cars");
    println!("Displaying {} cars", results.len());
    for car in results {
        println!("{}", car.model);
    }
}
