use orm_module::model::car::NewCar;

fn get_new_car() -> NewCar {
    let mut model = String::new();
    let mut year_std = String::new();

    println!("Which model do you want to load?");
    std::io::stdin().read_line(&mut model).unwrap();
    println!("Which year is the new model?");
    std::io::stdin().read_line(&mut year_std).unwrap();

    let year = year_std.trim().parse().expect("Couldn't parse the year");
    NewCar { model, year }
}

fn main() {
    let to_store = get_new_car();
    let mut conn = orm_module::establish_connection();
    let stored = to_store.create(&mut conn);
    println!("Stored car {:?}", stored);
}
