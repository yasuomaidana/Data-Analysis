use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub mod model;
pub mod schema;
pub mod with_recursive_no_union;

pub fn get_url() -> String {
    dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish_connection() -> PgConnection {
    let database_url = get_url();
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}\n", database_url))
}
