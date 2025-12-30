use diesel::pg::Pg;
use diesel::{QueryDsl, RunQueryDsl};
use orm_module::establish_connection;
use orm_module::schema::user_schema::users;

fn main() {
    let mut conn = establish_connection();
    let tuple = (users::name, users::email);
    let nested_tuple = (tuple.0, tuple.1, tuple);

    let query = users::table.select(nested_tuple);
    println!("SQL: {}", diesel::debug_query::<Pg, _>(&query));

    let name_email = query
        .load::<(String, String, (String, String))>(&mut conn)
        .expect("Error loading name");

    for nm in name_email {
        println!(
            "name {}, email: {} nested({}, {})",
            nm.0, nm.1, nm.2.0, nm.2.1
        );
    }
}
