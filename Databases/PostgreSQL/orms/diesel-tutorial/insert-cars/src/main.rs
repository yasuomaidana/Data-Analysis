use diesel::dsl::insert_into;
use diesel::{RunQueryDsl, SelectableHelper};
use orm_module::model::car::{Car, NewCar};
use orm_module::schema::car_schema::cars::dsl::cars;
use utils::reader::{get_arg, read_line};

fn get_new_car() -> NewCar {
    println!("Which model do you want to load?");
    let model: String = read_line().unwrap();
    println!("Which year is the new model?");
    let year: i32 = read_line().unwrap();
    NewCar { model, year }
}

fn main() {
    let mut conn = orm_module::establish_connection();

    let items = get_arg::<u32>(&std::env::args().collect(), 1);
    match items {
        Ok(Some(n)) => {
            let new_cars = (0..n).map(|_| get_new_car()).collect::<Vec<_>>();
            let stored_cars = insert_into(cars)
                .values(&new_cars)
                .returning(Car::as_returning())
                .get_results(&mut conn);
            println!("Stored cars: {:?}", stored_cars);
        }
        Ok(None) => {
            let to_store = get_new_car();
            let stored = to_store.create(&mut conn);
            println!("Stored car {:?}", stored);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
