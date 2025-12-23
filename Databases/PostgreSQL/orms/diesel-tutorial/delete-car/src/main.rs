use diesel::RunQueryDsl;
use diesel::{delete, QueryDsl};
use diesel::{BoolExpressionMethods, ExpressionMethods};
use orm_module::establish_connection;
use orm_module::model::car::Car;
use orm_module::schema::car_schema::cars::dsl::cars;
use orm_module::schema::car_schema::cars::{model, year};
use utils::reader::get_arg_or_prompt;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let model_input: String = get_arg_or_prompt(&args, 1, "Enter model: ");
    let year_input: i32 = get_arg_or_prompt(&args, 2, "Enter year: ");

    let mut connection = establish_connection();

    if year_input < 0 {
        println!("Delete all {model_input} cars");
        let stored_cars =
            delete(cars.filter(model.eq(model_input))).get_results::<Car>(&mut connection);

        match stored_cars {
            Ok(cars_) => {
                for car in cars_ {
                    println!("Deleted car: {:?}", car);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    } else {
        let stored_car = delete(cars.filter(year.eq(year_input).and(model.eq(model_input))))
            // you can use execute to only get a number instead of the car
            .get_result::<Car>(&mut connection);

        match stored_car {
            Ok(car) => {
                println!("Deleted car: {:?}", car);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
