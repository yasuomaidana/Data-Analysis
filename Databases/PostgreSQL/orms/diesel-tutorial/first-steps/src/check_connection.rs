use orm_module::establish_connection;

fn main() {
    let _ = &mut establish_connection();
    println!("Connection established");
}
