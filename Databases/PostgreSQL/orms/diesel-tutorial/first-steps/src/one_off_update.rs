use diesel::QueryDsl;
use diesel::{BoolExpressionMethods, ExpressionMethods, RunQueryDsl, update};
use orm_module::establish_connection;
use orm_module::model::car::Car;
use orm_module::schema::car_schema::cars::dsl::cars;
use orm_module::schema::car_schema::cars::{model, year};
use utils::reader::{get_arg_or_default, get_arg_or_prompt};

fn main() {
    println!("Updating car using modifying struct approach");
    let args: Vec<String> = std::env::args().collect();
    let model_input: String = get_arg_or_prompt(&args, 1, "Enter model: ");
    let year_input: i32 = get_arg_or_default(&args, 2, 2015);

    let mut connection = establish_connection();

    let updated = update(cars.filter(model.eq(&model_input).and(year.eq(year_input))))
        .set(year.eq(get_arg_or_default(&args, 3, 2015)))
        .get_result::<Car>(&mut connection);

    match updated {
        Ok(car) => {
            println!("Updated {:?}", car);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
