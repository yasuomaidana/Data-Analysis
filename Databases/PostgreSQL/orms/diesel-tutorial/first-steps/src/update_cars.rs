use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    SaveChangesDsl, SelectableHelper,
};
use orm_module::establish_connection;
use orm_module::model::car::Car;
use orm_module::schema::car_schema::cars::dsl::cars;
use orm_module::schema::car_schema::cars::{model, year};
use utils::reader::{get_arg_or_default, get_arg_or_prompt};

fn main() {
    let mut connection = establish_connection();
    println!("Updating car using modifying struct approach");
    let args: Vec<String> = std::env::args().collect();
    let model_input: String = get_arg_or_prompt(&args, 1, "Enter model: ");
    let year_input: i32 = get_arg_or_default(&args, 2, 2015);

    // Use `Car::as_select()` so the projection explicitly matches the `Car` struct.
    // This improves type safety and is required for joins or custom projections.
    let stored_car = cars
        .filter(model.eq(model_input).and(year.eq(year_input)))
        .select(Car::as_select())
        .first::<Car>(&mut connection)
        .optional();

    match stored_car {
        Ok(Some(mut stored_car)) => {
            println!("Updating car: {:?}", stored_car);

            let new_year: i32 = get_arg_or_prompt(&args, 3, "Enter new year: ");

            stored_car.year = new_year;
            let new: Car = stored_car
                .save_changes(&mut connection)
                .expect("Error updating car");
            println!("Updated car: {:?}", new);
        }
        Ok(None) => {
            println!("No car found");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
