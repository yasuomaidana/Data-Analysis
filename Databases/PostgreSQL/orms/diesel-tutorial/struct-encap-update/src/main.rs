// use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
// use diesel::dsl::update;
// use orm_module::establish_connection;
// use orm_module::model::car::Car;
// use orm_module::schema::car_schema::cars::dsl::cars;
// use orm_module::schema::car_schema::cars::{model, year};
// use utils::reader::{get_arg_or_default, get_arg_or_prompt};

fn main() {
    println!("Updating car using struct encapsulation");
    // TODO: Implementation deferred — requires many-to-many (m-to-n) relationship support in ORM/schema.
    // Once m-to-n associations are defined, implement the update using struct encapsulation and handle results.

    // let args: Vec<String> = std::env::args().collect();
    //
    // let model_input: String = get_arg_or_prompt(&args, 1, "Enter model: ");
    // let year_input: i32 = get_arg_or_default(&args, 2, 2015);
    //
    // let mut connection = establish_connection();
    //
    // let stored_car = cars
    //     .filter(year.eq(year_input).and(model.eq(model_input)))
    //     .get_result::<Car>(&mut connection);
    //
    // match stored_car {
    //     Ok(car) => {
    //         println!("Found car: {:?}", car);
    //         update(Car::belonging_to(car))
    //     }
    //     Err(e) => {
    //
    //     }
    // }
}
