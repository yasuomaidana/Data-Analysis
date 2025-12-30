use diesel::prelude::*;
use orm_module::establish_connection;
use orm_module::model::car::Car;
use orm_module::schema::car_schema::cars::dsl::cars;
use orm_module::schema::car_schema::cars::year;

fn main() {
    let connection = &mut establish_connection();

    let args: Vec<String> = std::env::args().collect();
    let filter_year: i32 = args
        .get(1)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(2015);

    let limit: i64 = args.get(2).and_then(|s| s.parse::<i64>().ok()).unwrap_or(5);

    let results = cars
        .filter(year.ge(filter_year))
        .limit(limit)
        .order_by(year.asc())
        .select(Car::as_select())
        .load(connection)
        .expect("Error loading cars");
    println!("Displaying {} cars", results.len());
    for car in results {
        println!("{}-{}", car.model.trim_end(), car.year);
    }
}
