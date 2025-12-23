use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
    SaveChangesDsl,
};
use orm_module::establish_connection;
use orm_module::model::car::Car;
use orm_module::schema::car_schema::cars::dsl::cars;
use orm_module::schema::car_schema::cars::{model, year};
use std::io::{self, Write};

fn main() {
    let mut connection = establish_connection();
    let args: Vec<String> = std::env::args().collect();
    let model_input: String = args
        .get(1)
        .and_then(|s| {
            let t = s.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        })
        .unwrap_or_else(|| {
            print!("Enter model: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().to_string()
        });
    let year_input: i32 = args
        .get(2)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(2015);

    let stored_car = cars
        .filter(model.eq(model_input).and(year.eq(year_input)))
        .first::<Car>(&mut connection)
        .optional();

    match stored_car {
        Ok(Some(mut stored_car)) => {
            println!("Updating car: {:?}", stored_car);

            let new_year: i32 = args
                .get(3)
                .and_then(|s| {
                    let t = s.trim();
                    if t.is_empty() {
                        None
                    } else {
                        t.parse::<i32>().ok()
                    }
                })
                .unwrap_or_else(|| {
                    print!("Enter new year: ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    input.trim().parse::<i32>().unwrap_or(2015)
                });

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
