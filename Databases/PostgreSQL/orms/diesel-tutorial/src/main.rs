mod schema;

use dotenvy::dotenv;
use std::env;

fn main() {
    match env::var("ENV_CONFIG") {
        Ok(env_file) => {
            dotenvy::from_path(env_file).ok();
        }
        Err(_) => {
            dotenv().ok();
        }
    }
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", db_url);
}
